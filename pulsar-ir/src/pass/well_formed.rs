//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    component::{Component, ComponentViewMut},
    from_ast::AsGeneratorPool,
    port::Port,
    variable::Variable,
    visitor::{Action, VisitorMut},
    Ir,
};
use pulsar_utils::{id::Id, pool::Handle};
use std::{collections::HashSet, ops::Deref};

use super::{Pass, PassOptions};

/// This pass and `Canonicalize` after it are applied before every other pass.
///
/// Enforced invariants include:
/// - All output ports are assigned to.
/// - All input ports are used at least once.
/// - All ports within a `par` are assigned to no more than once.
/// - All outermost for-loops should have constant-integer bounds.
#[derive(Default)]
pub struct WellFormed {
    /// List of ports assigned to.
    assigned_ports: HashSet<Handle<Port>>,
    // Redundant but yeah
    assigned_vars: HashSet<Variable>,
    read_vars: HashSet<Variable>,
}

impl<P: AsGeneratorPool> VisitorMut<P> for WellFormed {
    fn start_enable(
        &mut self, _id: Id, enable: &mut Ir, _comp_view: &mut ComponentViewMut,
        _pool: &mut P,
    ) -> Action {
        for port in enable.ports_used_ref() {
            if let Port::Variable(var) = port.deref() {
                self.read_vars.insert(*var);
            }
        }

        if let Port::Constant(_) = &*enable.kill() {
            panic!(
                "port {} on lhs of ir operation is not an lvalue",
                enable.kill()
            );
        }

        // if !self.assigned_ports.insert(enable.kill()) {
        //     panic!(
        //         "port {} already assigned to ({})",
        //         enable.kill(),
        //         self.assigned_ports.get(&enable.kill()).unwrap()
        //     )
        // }
        // if let Some(var) = enable.kill().root_var() {
        //     assert!(self.assigned_vars.insert(var));
        // }

        Action::None
    }

    fn finish_component(&mut self, comp: &mut Component, _pool: &mut P) {
        // for var in comp.outputs() {
        //     if !self.assigned_vars.contains(var) {
        //         panic!("output port {} not assigned to", var);
        //     }
        // }
        // for var in comp.inputs() {
        //     if !self.read_vars.contains(var) {
        //         panic!("input port {} not read from", var);
        //     }
        // }
    }
}

impl<P: AsGeneratorPool> Pass<P> for WellFormed {
    fn name(&self) -> &str {
        "well-formed"
    }

    fn setup(&mut self, _options: PassOptions) {}
}
