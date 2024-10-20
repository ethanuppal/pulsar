//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    component::{Component, ComponentViewMut},
    control::{Control, Par},
    from_ast::AsGeneratorPool,
    port::Port,
    variable::Variable,
    visitor::{Action, VisitorMut},
    Ir
};
use core::panic;
use pulsar_frontend::type_inferer::AffineEnvironment;
use pulsar_utils::{id::Id, pool::Handle};
use std::collections::HashSet;

use super::{Pass, PassOptions};

/// This pass and `Canonicalize` after it are applied before every other pass.
///
/// Enforced invariants include:
/// - All output ports are assigned to.
/// - All input ports are used at least once.
/// - All ports within a `par` are assigned to no more than once.
/// - All outermost for-loops should have constant-integer bounds.
pub struct WellFormed {
    read_vars: HashSet<Variable>,
    written_vars: HashSet<Variable>,
    par_disjoint_env: AffineEnvironment<Id, Handle<Port>>
}

impl Default for WellFormed {
    fn default() -> Self {
        Self {
            read_vars: HashSet::default(),
            written_vars: HashSet::default(),
            par_disjoint_env: AffineEnvironment::new("bug")
        }
    }
}

impl<P: AsGeneratorPool> VisitorMut<P> for WellFormed {
    fn start_par(
        &mut self, _id: Id, _par: &mut Par, _comp_view: &mut ComponentViewMut,
        _pool: &mut P
    ) -> Action {
        self.par_disjoint_env.enter_local();
        Action::None
    }

    fn finish_par(
        &mut self, _id: Id, _par: &mut Par, _comp_view: &mut ComponentViewMut,
        __pool: &mut P
    ) -> Action {
        self.par_disjoint_env.exit_local();
        Action::None
    }

    fn start_enable(
        &mut self, id: Id, enable: &mut Ir, _comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        for port in enable.gen_used() {
            for var in port.vars() {
                self.read_vars.insert(var);
            }
        }

        if let Port::Constant(_) = &*enable.kill() {
            panic!(
                "port {} on lhs of ir operation is not an lvalue",
                enable.kill()
            );
        } else if let Some(kill_var) = enable.kill_var() {
            self.written_vars.insert(kill_var);
        }

        self.par_disjoint_env
            .take(id, enable.kill())
            .unwrap_or_else(|error| {
                panic!(
                    "{} tried to write to {} in the same par that {} did",
                    enable,
                    enable.kill(),
                    Handle::<Control>::from_id(error.owner, pool)
                );
            });

        Action::None
    }

    fn finish_component(&mut self, comp: &mut Component, _pool: &mut P) {
        for var in comp.outputs() {
            if !self.written_vars.contains(var) {
                panic!("output port {} not assigned to", var);
            }
            if self.read_vars.contains(var) {
                panic!("cannot read output port {}", var);
            }
        }
        for var in comp.inputs() {
            if !self.read_vars.contains(var) {
                panic!("input port {} not read from", var);
            }
            if self.written_vars.contains(var) {
                panic!("cannot write to input port {}", var);
            }
        }
    }
}

impl<P: AsGeneratorPool> Pass<P> for WellFormed {
    fn name() -> &'static str {
        "well-formed"
    }

    fn from(
        _options: PassOptions, __comp: &mut Component, _pool: &mut P
    ) -> Self {
        Self::default()
    }
}
