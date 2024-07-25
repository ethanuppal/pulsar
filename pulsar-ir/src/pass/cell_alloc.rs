//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    cell::Cell,
    component::ComponentViewMut,
    control::For,
    from_ast::AsGeneratorPool,
    port::Port,
    visitor::{Action, Visitor},
    Ir
};

pub struct CellAlloc;

impl<P: AsGeneratorPool> Visitor<P> for CellAlloc {
    fn start_for(
        &mut self, for_: &mut For, comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        let index_reg_bits = match for_.exclusive_upper_bound() {
            Port::Constant(value) => {
                64 - ((value - 1).leading_zeros() as usize)
            }
            _ => 64
        };
        comp_view
            .cell_alloc
            .insert(for_.variant(), pool.add(Cell::Register(index_reg_bits)));
        Action::None
    }

    fn start_enable(
        &mut self, enable: &mut Ir, comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        if let Port::Variable(var) = *enable.kill() {
            comp_view
                .cell_alloc
                .insert(var, pool.add(Cell::Register(64)));
        }
        Action::None
    }
}
