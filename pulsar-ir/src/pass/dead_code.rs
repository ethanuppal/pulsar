//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use std::{
    collections::{HashMap, HashSet},
    mem::MaybeUninit,
    ops::Deref
};

use either::Either::{self, Left, Right};
use pulsar_utils::{id::Id, pool::Handle};

use crate::{
    component::Component,
    control::{Control, For, Par, Seq},
    from_ast::AsGeneratorPool,
    operand::Operand,
    variable::Variable,
    visitor::{Action, Visitor}
};

/// Conservative dead-code elimination. If an operand assigned to does not
/// appear within its scope, and it is not a component port, then the assignment
/// in which it appears is eliminated.
#[derive(Default)]
pub struct DeadCode {
    comp_ports: Vec<Variable>,
    gen_sets: HashMap<Id, HashSet<Operand>>
}

impl DeadCode {
    /// Computes the gen set for a control with id `id` and children `children`.
    ///
    /// Precondition: all non-enable control in `children` already has a gen set
    /// computed.
    fn calculate_gen_set(
        &mut self, id: Id, children: &mut Vec<Handle<Control>>
    ) {
        // bad software engineering here lol
        let mut gen_set = HashSet::new();
        for child in children {
            if let Control::Enable(ir) = (*child).deref() {
                for operand in ir.gen() {
                    gen_set.insert(operand.clone());
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

    /// Removes all assignments with unused operands.
    ///
    /// Precondition: the gen set exists for the control with id `id` and
    /// children `children` (aka [`DeadCode::calculate_gen_set`] or
    /// [`DeadCode::extend_gen_set`]).
    fn dead_code(&self, id: Id, children: &mut Vec<Handle<Control>>) {
        let gen_set = self
            .gen_sets
            .get(&id)
            .expect("DeadCode::calculate_gen_set was not called");
        for i in (0..children.len()).rev() {
            if let Control::Enable(ir) = &*children[i] {
                let kill = ir.kill();
                if !gen_set.contains(&kill) && {
                    if let Some(kill_var) = kill.gen_var() {
                        !self.comp_ports.contains(&kill_var)
                    } else {
                        true
                    }
                } {
                    children.remove(i);
                }
            }
        }
    }
}

impl<P: AsGeneratorPool> Visitor<P> for DeadCode {
    fn start_component(&mut self, comp: &mut Component) {
        log::warn!(
            "You're using the dead code pass, which is currently unstable"
        );

        self.comp_ports = comp
            .inputs()
            .iter()
            .chain(comp.outputs())
            .map(|(var, _)| var)
            .cloned()
            .collect();
    }

    fn finish_for(&mut self, for_: &mut For, _pool: &mut P) -> Action {
        self.extend_gen_set(for_.id, for_.body.id());
        Action::None
    }

    fn finish_seq(&mut self, seq: &mut Seq, _pool: &mut P) -> Action {
        self.calculate_gen_set(seq.id, &mut seq.children);
        self.dead_code(seq.id, &mut seq.children);
        Action::None
    }

    fn finish_par(&mut self, par: &mut Par, _pool: &mut P) -> Action {
        self.calculate_gen_set(par.id, &mut par.children);
        self.dead_code(par.id, &mut par.children);
        Action::None
    }
}
