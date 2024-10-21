//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    component::{Component, ComponentViewMut},
    control::{Control, For, IfElse, Par, Seq},
    from_ast::AsGeneratorPool,
    port::Port,
    visitor::{Action, Visitor},
    Ir
};
use core::panic;
use pulsar_utils::{id::Id, pool::Handle};
use std::{cmp, collections::HashMap, mem, ptr::null};

use super::Analysis;

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

    pub fn times(self, factor: usize) -> Timing {
        Timing::pipelined(self.init_interval, self.latency * factor)
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
    pub fn for_control<P: AsGeneratorPool>(
        control: Handle<Control>, pool: &P
    ) -> Self {
        let mut new_self = Self::default();
        let fake = unsafe { mem::transmute(0u64) };
        new_self.traverse_control(control, fake, pool, false);
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

impl<P: AsGeneratorPool> Visitor<P> for TimingAnalysis {
    fn finish_enable(
        &mut self, id: Id, enable: &Ir, _comp: &Component, _pool: &P
    ) {
        log::trace!("at ir {}: {}", id, enable);
        self.set(
            id,
            match enable {
                Ir::Add(_, _, _) => Timing::combinational(),
                Ir::Mul(_, _, _) => MULTIPLY_TIMING,
                Ir::Assign(_, _) => Timing::combinational()
            }
        );
    }

    fn finish_delay(
        &mut self, id: Id, delay: &usize, _comp: &Component, _pool: &P
    ) {
        self.set(id, Timing::sequential(*delay));
    }

    fn finish_for(&mut self, id: Id, for_: &For, _comp: &Component, pool: &P) {
        // TODO: we assume constant integer bounds
        let (
            Port::Constant(lower_bound),
            Port::Constant(exclusive_upper_bound)
        ) = (for_.lower_bound(), for_.exclusive_upper_bound())
        else {
            panic!("we assume constant integer bounds");
        };

        self.set(
            id,
            Timing::sequential(for_.init_latency()).then(
                self.get(for_.body().id_in(pool))
                    .times((*exclusive_upper_bound - *lower_bound) as usize)
            )
        );
    }

    fn finish_seq(&mut self, id: Id, seq: &Seq, _comp: &Component, pool: &P) {
        let timing = seq
            .children()
            .iter()
            .map(|child| self.get(child.id_in(pool)))
            .reduce(Timing::compose)
            .unwrap_or_default();
        self.set(id, timing);
    }

    fn finish_par(&mut self, id: Id, par: &Par, _comp: &Component, pool: &P) {
        let timing = par
            .children()
            .iter()
            .map(|child| self.get(child.id_in(pool)))
            .reduce(Timing::max)
            .unwrap_or_default();
        self.set(id, timing);
    }

    fn finish_if_else(
        &mut self, id: Id, if_else: &IfElse, _comp: &Component, pool: &P
    ) {
        self.set(
            id,
            Timing::max(
                self.get(if_else.true_branch.id_in(pool)),
                self.get(if_else.false_branch.id_in(pool))
            )
        );
    }
}

impl Analysis for TimingAnalysis {}
