//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    component::{Component, ComponentViewMut},
    control::{For, IfElse, Par, Seq},
    from_ast::AsGeneratorPool,
    port::Port,
    visitor::{Action, VisitorMut},
    Ir
};
use pulsar_utils::{id::Id, pool::Handle};
use std::collections::HashSet;

/// Identifies ports and control whose modification/execution have side effects.
#[derive(Default)]
pub struct SideEffectAnalysis {
    pub effectual_ports: HashSet<Handle<Port>>,
    pub effectual_control: HashSet<Id>
}

impl SideEffectAnalysis {
    pub fn from<P: AsGeneratorPool>(
        comp: &mut Component, pool: &mut P
    ) -> Self {
        let mut new_self = Self::default();
        new_self.traverse_component(comp, pool, true);
        new_self
    }
}

impl<P: AsGeneratorPool> VisitorMut<P> for SideEffectAnalysis {
    fn start_component(&mut self, comp: &mut Component, pool: &mut P) {
        self.effectual_ports = comp
            .outputs()
            .iter()
            .map(|var| pool.add(Port::Variable(*var)))
            .collect();
    }

    fn start_delay(
        &mut self, id: Id, delay: &mut usize,
        _comp_view: &mut ComponentViewMut, _pool: &mut P
    ) -> Action {
        if *delay > 0 {
            self.effectual_control.insert(id);
        }
        Action::None
    }

    fn finish_enable(
        &mut self, id: Id, enable: &mut Ir, _comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        log::trace!("reached ir: {}", enable);
        if let Some(kill_var) = enable.kill_var() {
            log::trace!("kill var found: {}", kill_var);
            if self
                .effectual_ports
                .contains(&pool.add(Port::Variable(kill_var)))
            {
                self.effectual_ports.extend(enable.gen_used());
                self.effectual_control.insert(id);
            }
        }
        Action::None
    }

    fn finish_seq(
        &mut self, id: Id, seq: &mut Seq, _comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        if seq.children().iter().any(|control| {
            self.effectual_control.contains(&control.id_in(pool))
        }) {
            self.effectual_control.insert(id);
        }
        Action::None
    }

    fn finish_par(
        &mut self, id: Id, par: &mut Par, _comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        if par.children().iter().any(|control| {
            self.effectual_control.contains(&control.id_in(pool))
        }) {
            self.effectual_control.insert(id);
        }
        Action::None
    }

    fn finish_for(
        &mut self, id: Id, for_: &mut For, _comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        if self.effectual_control.contains(&for_.body().id_in(pool)) {
            self.effectual_control.insert(id);
        }
        Action::None
    }

    fn finish_if_else(
        &mut self, id: Id, if_else: &mut IfElse,
        _comp_view: &mut ComponentViewMut, pool: &mut P
    ) -> Action {
        if self
            .effectual_control
            .contains(&if_else.true_branch.id_in(pool))
            || self
                .effectual_control
                .contains(&if_else.false_branch.id_in(pool))
        {
            self.effectual_control.insert(id);
        }
        Action::None
    }

    fn finish_component(&mut self, _comp: &mut Component, _pool: &mut P) {
        for port in &self.effectual_ports {
            log::trace!("effectual port: {}", port);
        }
    }
}
