//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    component::{Component, ComponentViewMut},
    control::{Control, For, IfElse, Par, Seq},
    from_ast::AsGeneratorPool,
    port::Port,
    variable::Variable,
    visitor::{Action, VisitorMut},
    Ir,
};
use pulsar_utils::{id::Id, pool::Handle};
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

use super::{Pass, PassOptions};

/// Conservative dead-code elimination. If an port assigned to does not
/// appear within its scope, and it is not an output port, then the assignment
/// in which it appears is eliminated. This is possible because of the IR
/// invariant that lvalues appear immediately on the kill-side of IR operations.
/// Due to bad design on my part, each IR operation does not get an id, so dead
/// code messes up whenever there are at least two reads of a variable, even if
/// that variable is actually dead.
///
/// Requires that `CalculateTiming` has been run.
#[derive(Default)]
pub struct DeadCode {
    preserve_timing: bool,
    output_ports: Vec<Variable>,
    gen_sets: HashMap<Id, HashSet<Handle<Port>>>,
}

impl DeadCode {
    /// Computes the gen set for a control with id `id` and children `children`.
    ///
    /// Precondition: all non-enable control in `children` already has a gen set
    /// computed.
    fn calculate_gen_set<P: AsGeneratorPool>(
        &mut self, id: Id, children: &mut Vec<Handle<Control>>, pool: &P,
    ) {
        // bad software engineering here lol. see main struct doc comment for
        // why it's slightly broken
        let mut gen_set = HashSet::new();
        for child in children {
            if let Control::Enable(ir) = (*child).deref() {
                for port in ir.gen() {
                    gen_set.insert(port);
                }
            } else if !matches!(**child, Control::Empty | Control::Delay(_)) {
                // flatten and remove original child set
                gen_set.extend(
                    self.gen_sets
                        .remove_entry(&child.id_in(pool))
                        .expect("precondition")
                        .1,
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
    fn dead_code<P: AsGeneratorPool>(
        &self, id: Id, children: &mut Vec<Handle<Control>>, pool: &P,
    ) -> Action {
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
                    if self.preserve_timing {
                        children.remove(i);
                    } else {
                        let latency = *pool.get_metadata(children[i]);
                        *children[i] = Control::Delay(latency);
                    }
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

impl<P: AsGeneratorPool> VisitorMut<P> for DeadCode {
    fn start_component(&mut self, comp: &mut Component, _pool: &mut P) {
        log::warn!(
            "You're using the dead code pass, which is currently unstable"
        );

        self.output_ports = comp.outputs().to_vec();

        log::trace!("BEFORE DEAD CODE:");
        log::trace!("{}", comp);
    }

    fn finish_for(
        &mut self, id: Id, for_: &mut For, _comp_view: &mut ComponentViewMut,
        pool: &mut P,
    ) -> Action {
        self.extend_gen_set(id, for_.body.id_in(pool));
        Action::None
    }

    fn finish_if_else(
        &mut self, id: Id, if_else: &mut IfElse,
        _comp_view: &mut ComponentViewMut, pool: &mut P,
    ) -> Action {
        self.extend_gen_set(id, if_else.true_branch.id_in(pool));
        self.extend_gen_set(id, if_else.false_branch.id_in(pool));
        Action::None
    }

    fn finish_seq(
        &mut self, id: Id, seq: &mut Seq, _comp_view: &mut ComponentViewMut,
        pool: &mut P,
    ) -> Action {
        self.calculate_gen_set(id, &mut seq.children, pool);
        self.dead_code(id, &mut seq.children, pool)
    }

    fn finish_par(
        &mut self, id: Id, par: &mut Par, _comp_view: &mut ComponentViewMut,
        pool: &mut P,
    ) -> Action {
        self.calculate_gen_set(id, &mut par.children, pool);
        self.dead_code(id, &mut par.children, pool)
    }
}

impl<P: AsGeneratorPool> Pass<P> for DeadCode {
    fn name(&self) -> &str {
        "dead-code"
    }

    fn setup(&mut self, options: PassOptions) {
        self.preserve_timing = options.contains(PassOptions::PRESERVE_TIMING);
    }
}
