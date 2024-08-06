//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::{Pass, PassOptions};
use crate::{
    analysis::{side_effect::SideEffectAnalysis, timing::TimingAnalysis},
    component::{Component, ComponentViewMut},
    control::{Control, For, IfElse, Par, Seq},
    from_ast::AsGeneratorPool,
    visitor::{Action, VisitorMut},
    Ir
};
use pulsar_utils::id::Id;

/// Conservative dead-code elimination (as per [`SideEffectAnalysis`]).
#[derive(Default)]
pub struct DeadCode {
    preserve_timing: bool,
    side_effects: SideEffectAnalysis,
    timing: TimingAnalysis
}

impl DeadCode {
    fn dead_code(&self, id: Id) -> Action {
        if self.side_effects.effectual_control.contains(&id) {
            return Action::None;
        }
        if self.preserve_timing {
            if let Some(latency) = self.timing.get(id).latency() {
                return Action::Replace(Control::Delay(latency.get()));
            }
        }
        Action::Remove
    }
}

impl<P: AsGeneratorPool> VisitorMut<P> for DeadCode {
    fn start_enable(
        &mut self, id: Id, _enable: &mut Ir, _comp_view: &mut ComponentViewMut,
        _pool: &mut P
    ) -> Action {
        self.dead_code(id)
    }

    fn start_delay(
        &mut self, id: Id, _delay: &mut usize,
        _comp_view: &mut ComponentViewMut, _pool: &mut P
    ) -> Action {
        self.dead_code(id)
    }

    fn start_seq(
        &mut self, id: Id, _seq: &mut Seq, _comp_view: &mut ComponentViewMut,
        _pool: &mut P
    ) -> Action {
        self.dead_code(id)
    }

    fn start_par(
        &mut self, id: Id, _par: &mut Par, _comp_view: &mut ComponentViewMut,
        _pool: &mut P
    ) -> Action {
        self.dead_code(id)
    }

    fn start_for(
        &mut self, id: Id, _for_: &mut For, _comp_view: &mut ComponentViewMut,
        _pool: &mut P
    ) -> Action {
        self.dead_code(id)
    }

    fn start_if_else(
        &mut self, id: Id, _if_else: &mut IfElse,
        _comp_view: &mut ComponentViewMut, _pool: &mut P
    ) -> Action {
        self.dead_code(id)
    }
}

impl<P: AsGeneratorPool> Pass<P> for DeadCode {
    fn name() -> &'static str {
        "dead-code"
    }

    fn from(options: PassOptions, comp: &mut Component, pool: &mut P) -> Self {
        Self {
            preserve_timing: options.contains(PassOptions::PRESERVE_TIMING),
            side_effects: SideEffectAnalysis::from(comp, pool),
            timing: TimingAnalysis::from(comp, pool)
        }
    }
}
