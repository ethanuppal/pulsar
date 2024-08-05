//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use std::ops::Deref;

use pulsar_utils::id::Id;

use crate::{
    cell::Cell,
    component::{Component, ComponentViewMut},
    control::{Control, For, Seq},
    from_ast::AsGeneratorPool,
    port::Port,
    visitor::{Action, VisitorMut},
    Ir
};

use super::{Pass, PassOptions};

pub struct CellAlloc;

pub fn min_bits_to_represent(value: usize) -> usize {
    if value == 0 {
        0
    } else {
        64 - ((value - 1).leading_zeros() as usize)
    }
}

impl<P: AsGeneratorPool> VisitorMut<P> for CellAlloc {
    fn start_for(
        &mut self, id: Id, for_: &mut For, comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        let index_reg_bits = match for_.exclusive_upper_bound() {
            Port::Constant(value) => min_bits_to_represent(*value as usize),
            _ => 64
        };
        comp_view
            .cell_alloc
            .insert(for_.variant(), pool.add(Cell::Register(index_reg_bits)));
        Action::None
    }

    // TODO: fix this. it needs to identify common variables in sequence and
    // assign them a register. right now it just looks root level so if there
    // are any nested pars it won't work :(
    fn start_seq(
        &mut self, id: Id, seq: &mut Seq, comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        for child in &seq.children {
            if let Control::Enable(enable) = child.deref() {
                if let Port::Variable(var) = *enable.kill() {
                    comp_view
                        .cell_alloc
                        .insert(var, pool.add(Cell::Register(64)));
                }
            }
        }
        Action::ModifiedInternally
    }
}

impl<P: AsGeneratorPool> Pass<P> for CellAlloc {
    fn name() -> &'static str {
        "cell-alloc"
    }

    fn from(
        _options: PassOptions, _comp: &mut Component, _pool: &mut P
    ) -> Self {
        Self
    }
}
