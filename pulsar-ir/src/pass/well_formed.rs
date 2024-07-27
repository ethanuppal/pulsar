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
    Ir
};
use pulsar_utils::pool::Handle;
use std::collections::HashSet;

use super::Pass;

/// This pass and `Canonicalize` after it are applied before every other pass.
#[derive(Default)]
pub struct WellFormed {
    /// List of ports assigned to.
    assigned_ports: HashSet<Handle<Port>>,
    // Redundant but yeah
    assigned_vars: HashSet<Variable>
}

impl<P: AsGeneratorPool> VisitorMut<P> for WellFormed {
    fn start_enable(
        &mut self, enable: &mut Ir, _comp_view: &mut ComponentViewMut,
        _pool: &mut P
    ) -> Action {
        if let Port::Constant(_) = &*enable.kill() {
            panic!(
                "port {} on lhs of ir operation is not an lvalue",
                enable.kill()
            );
        }

        if !self.assigned_ports.insert(enable.kill()) {
            panic!(
                "port {} already assigned to ({})",
                enable.kill(),
                self.assigned_ports.get(&enable.kill()).unwrap()
            )
        }
        if let Some(var) = enable.kill().root_var() {
            assert!(self.assigned_vars.insert(var));
        }

        Action::None
    }

    fn finish_component(&mut self, comp: &mut Component, _pool: &mut P) {
        for (var, _) in comp.outputs() {
            if !self.assigned_vars.contains(var) {
                panic!("output port {} not assigned to", var);
            }
        }
    }
}

impl<P: AsGeneratorPool> Pass<P> for WellFormed {
    fn name(&self) -> &str {
        "well-formed"
    }
}
