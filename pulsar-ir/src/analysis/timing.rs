//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    component::{Component, ComponentViewMut},
    control::{For, IfElse, Par, Seq},
    from_ast::AsGeneratorPool,
    visitor::{Action, VisitorMut},
    Ir
};
use core::panic;
use pulsar_utils::id::Id;
use std::{cmp, collections::HashMap, num::NonZeroUsize};

/// Computes timing information for all non-empty control.
#[derive(Clone, Copy, Default)]
pub struct Timing {
    latency: Option<NonZeroUsize>
}

impl Timing {
    pub fn combinational() -> Self {
        Self { latency: None }
    }

    /// requires: `latency > 0`.
    pub fn sequential<S: Into<usize>>(latency: S) -> Self {
        Self {
            latency: Some(NonZeroUsize::new(latency.into()).unwrap())
        }
    }

    pub fn from_delay(delay: usize) -> Self {
        if delay == 0 {
            Self::combinational()
        } else {
            Self::sequential(delay)
        }
    }

    pub fn latency(&self) -> Option<NonZeroUsize> {
        self.latency
    }

    pub fn then(self, next: Self) -> Self {
        match (self.latency, next.latency) {
            (None, None) => Self::combinational(),
            (None, Some(latency)) | (Some(latency), None) => {
                Self::sequential(latency)
            }
            (Some(latency0), Some(latency1)) => {
                Self::sequential(latency0.get() + latency1.get())
            }
        }
    }
}

pub fn max(lhs: Timing, rhs: Timing) -> Timing {
    match ((lhs).latency, (rhs).latency) {
        (None, None) => Timing::combinational(),
        (Some(latency), None) | (None, Some(latency)) => {
            Timing::sequential(latency.get()).to_owned()
        }
        (Some(latency0), Some(latency1)) => {
            Timing::sequential(cmp::max(latency0.get(), latency1.get()))
        }
    }
}

#[derive(Default)]
pub struct TimingAnalysis(HashMap<Id, Timing>);

impl TimingAnalysis {
    pub fn from<P: AsGeneratorPool>(
        comp: &mut Component, pool: &mut P
    ) -> Self {
        let mut new_self = Self::default();
        new_self.traverse_component(comp, pool, false);
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
                Ir::Mul(_, _, _) => Timing::sequential(4usize),
                Ir::Assign(_, _) => Timing::combinational()
            }
        );
        Action::None
    }

    fn finish_delay(
        &mut self, id: Id, delay: &mut usize,
        _comp_view: &mut ComponentViewMut, _pool: &mut P
    ) -> Action {
        self.set(id, Timing::from_delay(*delay));
        Action::None
    }

    fn finish_for(
        &mut self, id: Id, for_: &mut For, _comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        self.set(
            id,
            Timing::from_delay(for_.init_latency())
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
            .reduce(|timing0, timing1| timing0.then(timing1))
            .unwrap_or_default();
        self.set(id, timing);
        Action::None
    }

    fn finish_par(
        &mut self, id: Id, par: &mut Par, _comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        log::trace!("at par: {} {}", par, self.0.contains_key(&1));
        let timing = par
            .children()
            .iter()
            .map(|child| self.get(child.id_in(pool)))
            .reduce(max)
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
            max(
                self.get(if_else.true_branch.id_in(pool)),
                self.get(if_else.false_branch.id_in(pool))
            )
        );
        Action::None
    }
}
