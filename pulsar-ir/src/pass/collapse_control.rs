//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use pulsar_utils::{id::Id, pool::Handle};

use crate::{
    component::ComponentViewMut,
    control::{Control, Par, Seq},
    from_ast::AsGeneratorPool,
    visitor::{Action, VisitorMut},
};
use std::mem;

use super::{Pass, PassOptions};

/// Collapses singleton or empty par/seq.
#[derive(Default)]
pub struct CollapseControl {
    preserve_timing: bool,
}

impl CollapseControl {
    fn collapse(&self, children: &mut Vec<Handle<Control>>) -> Action {
        for i in (0..children.len()).rev() {
            if matches!(*children[i], Control::Empty | Control::Delay(0)) {
                children.remove(i);
            }
        }
        match children.len() {
            0 => Action::Remove,
            1 => Action::Replace(mem::take(&mut children[0])),
            _ => Action::None,
        }
    }
}

impl<P: AsGeneratorPool> VisitorMut<P> for CollapseControl {
    fn finish_seq(
        &mut self, id: Id, seq: &mut Seq, _comp_view: &mut ComponentViewMut,
        _pool: &mut P,
    ) -> Action {
        self.collapse(&mut seq.children)
    }

    fn finish_par(
        &mut self, id: Id, par: &mut Par, _comp_view: &mut ComponentViewMut,
        _pool: &mut P,
    ) -> Action {
        self.collapse(&mut par.children)
    }
}

impl<P: AsGeneratorPool> Pass<P> for CollapseControl {
    fn name(&self) -> &str {
        "collapse-control"
    }

    fn setup(&mut self, options: PassOptions) {
        self.preserve_timing = options.contains(PassOptions::PRESERVE_TIMING);
    }
}
