//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    component::{Component, ComponentViewMut},
    control::{Control, For, IfElse, Par, Seq},
    from_ast::AsGeneratorPool,
    visitor::{Action, VisitorMut},
    Ir
};
use core::panic;
use pulsar_utils::{id::Id, pool::Handle};
use std::{cmp, collections::HashMap};

/// Computes timing information for all non-empty control.
///
/// TODO: currently assumes input is needed for just 1 cycle and available for
/// just 1 cycle
#[derive(Clone, Copy, Default)]
pub struct Timing {
    init_interval: usize,
    latency: usize
}

impl Timing {
    pub const fn pipelined(init_interval: usize, latency: usize) -> Self {
        Self {
            init_interval,
            latency
        }
    }

    pub fn combinational() -> Self {
        Self::pipelined(0, 0)
    }

    pub fn sequential(latency: usize) -> Self {
        Self::pipelined(0, latency)
    }

    pub fn init_interval(&self) -> usize {
        self.init_interval
    }

    pub fn latency(&self) -> usize {
        self.latency
    }

    pub fn compose(lhs: Timing, rhs: Timing) -> Timing {
        Timing::pipelined(
            cmp::max(lhs.init_interval(), rhs.init_interval()),
            lhs.latency() + rhs.latency()
        )
    }

    pub fn then(self, rhs: Timing) -> Timing {
        Timing::compose(self, rhs)
    }

    pub fn max(lhs: Timing, rhs: Timing) -> Timing {
        Timing::pipelined(
            cmp::max(lhs.init_interval(), rhs.init_interval()),
            cmp::max(lhs.latency(), rhs.latency())
        )
    }
}

/// dumb hack that also relies on calyx stdlib implementation
pub const MULTIPLY_TIMING: Timing = Timing::pipelined(1, 4);

#[derive(Default)]
pub struct TimingAnalysis(HashMap<Id, Timing>);

impl TimingAnalysis {
    pub fn for_comp<P: AsGeneratorPool>(
        comp: &mut Component, pool: &mut P
    ) -> Self {
        let mut new_self = Self::default();
        new_self.traverse_component(comp, pool, false);
        new_self
    }

    pub fn for_control<P: AsGeneratorPool>(
        control: Handle<Control>, pool: &mut P
    ) -> Self {
        let mut new_self = Self::default();
        let mut fake = unsafe { ComponentViewMut::undefined() };
        new_self.traverse_control(control, &mut fake, pool, false);
        new_self
    }

    pub fn get(&self, id: Id) -> Timing {
        self.0
            .get(&id)
            .cloned()
            .unwrap_or_else(|| panic!("should have already visited id {}", id))
    }

    fn set(&mut self, id: Id, timing: Timing) {
        self.0.insert(id, timing);
    }
}

impl<P: AsGeneratorPool> VisitorMut<P> for TimingAnalysis {
    fn finish_enable(
        &mut self, id: Id, enable: &mut Ir, _comp_view: &mut ComponentViewMut,
        _pool: &mut P
    ) -> Action {
        log::trace!("at ir {}: {}", id, enable);
        self.set(
            id,
            match enable {
                Ir::Add(_, _, _) => Timing::combinational(),
                Ir::Mul(_, _, _) => MULTIPLY_TIMING,
                Ir::Assign(_, _) => Timing::combinational()
            }
        );
        Action::None
    }

    fn finish_delay(
        &mut self, id: Id, delay: &mut usize,
        _comp_view: &mut ComponentViewMut, _pool: &mut P
    ) -> Action {
        self.set(id, Timing::sequential(*delay));
        Action::None
    }

    fn finish_for(
        &mut self, id: Id, for_: &mut For, _comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        self.set(
            id,
            Timing::sequential(for_.init_latency())
                .then(self.get(for_.body().id_in(pool)))
        );
        Action::None
    }

    fn finish_seq(
        &mut self, id: Id, seq: &mut Seq, _comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        let timing = seq
            .children()
            .iter()
            .map(|child| self.get(child.id_in(pool)))
            .reduce(Timing::compose)
            .unwrap_or_default();
        self.set(id, timing);
        Action::None
    }

    fn finish_par(
        &mut self, id: Id, par: &mut Par, _comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        let timing = par
            .children()
            .iter()
            .map(|child| self.get(child.id_in(pool)))
            .reduce(Timing::max)
            .unwrap_or_default();
        self.set(id, timing);
        Action::None
    }

    fn finish_if_else(
        &mut self, id: Id, if_else: &mut IfElse,
        _comp_view: &mut ComponentViewMut, pool: &mut P
    ) -> Action {
        self.set(
            id,
            Timing::max(
                self.get(if_else.true_branch.id_in(pool)),
                self.get(if_else.false_branch.id_in(pool))
            )
        );
        Action::None
    }
}
