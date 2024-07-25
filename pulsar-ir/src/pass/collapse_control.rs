//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use pulsar_utils::pool::Handle;

use crate::{
    component::ComponentViewMut,
    control::{Control, Par, Seq},
    from_ast::AsGeneratorPool,
    visitor::{Action, Visitor}
};
use std::mem;

/// Collapses singleton or empty par/seq.
pub struct CollapseControl;

impl CollapseControl {
    fn collapse(&self, children: &mut Vec<Handle<Control>>) -> Action {
        for i in (0..children.len()).rev() {
            if matches!(*children[i], Control::Empty) {
                children.remove(i);
            }
        }
        match children.len() {
            0 => Action::Remove,
            1 => Action::Replace(mem::take(&mut children[0])),
            _ => Action::None
        }
    }
}

impl<P: AsGeneratorPool> Visitor<P> for CollapseControl {
    fn finish_seq(
        &mut self, seq: &mut Seq, _comp_view: &mut ComponentViewMut, _pool: &mut P
    ) -> Action {
        self.collapse(&mut seq.children)
    }

    fn finish_par(
        &mut self, par: &mut Par, _comp_view: &mut ComponentViewMut, _pool: &mut P
    ) -> Action {
        self.collapse(&mut par.children)
    }
}
