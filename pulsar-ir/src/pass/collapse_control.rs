//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use pulsar_utils::{id::Id, pool::Handle};

use crate::{
    component::{Component, ComponentViewMut},
    control::{Control, For, Par, Seq},
    from_ast::AsGeneratorPool,
    port::Port,
    visitor::{Action, VisitorMut}
};
use std::{
    mem,
    ops::{Deref, DerefMut}
};

use super::{Pass, PassOptions};

/// Collapses singleton or empty par/seq.
#[derive(Default)]
pub struct CollapseControl {
    preserve_timing: bool
}

impl CollapseControl {
    fn collapse(&self, children: &mut Vec<Handle<Control>>) -> Action {
        for i in (0..children.len()).rev() {
            if matches!(*children[i], Control::Empty | Control::Delay(0)) {
                children.remove(i);
            } else if i > 0
                && matches!(
                    (&*children[i - 1], &*children[i]),
                    (Control::Delay(..), Control::Delay(..))
                )
            {
                let (Control::Delay(a), Control::Delay(b)) = (
                    mem::take(children[i - 1].deref_mut()),
                    mem::take(children[i].deref_mut())
                ) else {
                    unreachable!();
                };
                children.remove(i);
                *children[i - 1].deref_mut() = Control::Delay(a + b);
            }
        }
        match children.len() {
            0 => Action::Remove,
            1 => Action::Replace(mem::take(&mut children[0])),
            _ => Action::None
        }
    }
}

impl<P: AsGeneratorPool> VisitorMut<P> for CollapseControl {
    fn finish_seq(
        &mut self, _id: Id, seq: &mut Seq, _comp_view: &mut ComponentViewMut,
        _pool: &mut P
    ) -> Action {
        self.collapse(&mut seq.children)
    }

    fn finish_par(
        &mut self, _id: Id, par: &mut Par, _comp_view: &mut ComponentViewMut,
        _pool: &mut P
    ) -> Action {
        self.collapse(&mut par.children)
    }

    fn finish_for(
        &mut self, _id: Id, for_: &mut For, _comp_view: &mut ComponentViewMut,
        _pool: &mut P
    ) -> Action {
        let Port::Constant(lower_bound) = for_.lower_bound() else {
            return Action::None;
        };
        let Port::Constant(upper_bound) = for_.exclusive_upper_bound() else {
            return Action::None;
        };
        let num_iters = if lower_bound > upper_bound {
            0
        } else {
            upper_bound - lower_bound
        };
        if matches!(for_.body().deref(), Control::Empty) && num_iters == 0 {
            return if self.preserve_timing {
                Action::Replace(Control::Delay(for_.init_latency()))
            } else {
                Action::Remove
            };
        }
        Action::None
    }
}

impl<P: AsGeneratorPool> Pass<P> for CollapseControl {
    fn name() -> &'static str {
        "collapse-control"
    }

    fn from(
        options: PassOptions, _comp: &mut Component, _pool: &mut P
    ) -> Self {
        Self {
            preserve_timing: options.contains(PassOptions::PRESERVE_TIMING)
        }
    }
}
