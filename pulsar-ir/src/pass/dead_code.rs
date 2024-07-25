//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use std::{
    collections::{HashMap, HashSet},
    ops::Deref
};

use pulsar_utils::{id::Id, pool::Handle};

use crate::{
    component::Component,
    control::{Control, For, IfElse, Par, Seq},
    from_ast::AsGeneratorPool,
    port::Port,
    variable::Variable,
    visitor::{Action, Visitor}
};

/// Conservative dead-code elimination. If an port assigned to does not
/// appear within its scope, and it is not an output port, then the assignment
/// in which it appears is eliminated. This is possible because of the IR
/// invariant that lvalues appear immediately on the kill-side of IR operations.
/// Due to bad design on my part, each IR operation does not get an id, so dead
/// code messes up whenever there are at least two reads of a variable, even if
/// that variable is actually dead.
#[derive(Default)]
pub struct DeadCode {
    output_ports: Vec<Variable>,
    gen_sets: HashMap<Id, HashSet<Handle<Port>>>
}

impl DeadCode {
    /// Computes the gen set for a control with id `id` and children `children`.
    ///
    /// Precondition: all non-enable control in `children` already has a gen set
    /// computed.
    fn calculate_gen_set(
        &mut self, id: Id, children: &mut Vec<Handle<Control>>
    ) {
        // bad software engineering here lol. see main struct doc comment for
        // why it's slightly broken
        let mut gen_set = HashSet::new();
        for child in children {
            if let Control::Enable(ir) = (*child).deref() {
                for port in ir.gen() {
                    gen_set.insert(port);
                }
            } else if !matches!(**child, Control::Empty) {
                // flatten and remove original child set
                gen_set.extend(
                    self.gen_sets
                        .remove_entry(&child.id())
                        .expect("precondition")
                        .1
                );
            }
        }
        self.gen_sets.insert(id, gen_set);
    }

    /// Adds all of the gen set of `child` to that of the control with id `id`,
    /// creating it if it does not exist.
    ///
    /// Requires: `child` has a gen set calcuated for it.
    fn extend_gen_set(&mut self, id: Id, child: Id) {
        let child_gen_set =
            self.gen_sets.get(&child).expect("precondition").clone();
        self.gen_sets.entry(id).or_default().extend(child_gen_set);
    }

    /// Removes all assignments with unused ports.
    ///
    /// Precondition: the gen set exists for the control with id `id` and
    /// children `children` (aka [`DeadCode::calculate_gen_set`] or
    /// [`DeadCode::extend_gen_set`]).
    fn dead_code(&self, id: Id, children: &mut Vec<Handle<Control>>) -> Action {
        let gen_set = self
            .gen_sets
            .get(&id)
            .expect("DeadCode::calculate_gen_set was not called");
        let mut did_modify = false;
        for i in (0..children.len()).rev() {
            if let Control::Enable(ir) = &*children[i] {
                let kill = ir.kill();
                if !gen_set.contains(&kill)
                    && kill
                        .vars()
                        .iter()
                        .all(|kill_var| !self.output_ports.contains(kill_var))
                {
                    children.remove(i);
                    did_modify = true;
                }
            }
        }
        if did_modify {
            Action::ModifiedInternally
        } else {
            Action::None
        }
    }
}

impl<P: AsGeneratorPool> Visitor<P> for DeadCode {
    fn start_component(&mut self, comp: &mut Component, _pool: &mut P) {
        log::warn!(
            "You're using the dead code pass, which is currently unstable"
        );

        self.output_ports =
            comp.outputs().iter().map(|(var, _)| var).cloned().collect();
    }

    fn finish_for(&mut self, for_: &mut For, _pool: &mut P) -> Action {
        self.extend_gen_set(for_.id, for_.body.id());
        Action::None
    }

    fn finish_if_else(
        &mut self, if_else: &mut IfElse, _pool: &mut P
    ) -> Action {
        self.extend_gen_set(if_else.id, if_else.true_branch.id());
        self.extend_gen_set(if_else.id, if_else.false_branch.id());
        Action::None
    }

    fn finish_seq(&mut self, seq: &mut Seq, _pool: &mut P) -> Action {
        self.calculate_gen_set(seq.id, &mut seq.children);
        self.dead_code(seq.id, &mut seq.children)
    }

    fn finish_par(&mut self, par: &mut Par, _pool: &mut P) -> Action {
        self.calculate_gen_set(par.id, &mut par.children);
        self.dead_code(par.id, &mut par.children)
    }
}
