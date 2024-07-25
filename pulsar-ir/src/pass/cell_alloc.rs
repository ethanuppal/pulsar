//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use std::mem;

use crate::{
    cell::Cell,
    component::Component,
    control::For,
    from_ast::AsGeneratorPool,
    port::Port,
    variable::Variable,
    visitor::{Action, Visitor},
    Ir
};

#[derive(Default)]
pub struct CellAlloc {
    allocs: Vec<(Variable, Cell)>
}

impl<P: AsGeneratorPool> Visitor<P> for CellAlloc {
    fn start_for(&mut self, for_: &mut For, _pool: &mut P) -> Action {
        let index_reg_bits = match for_.exclusive_upper_bound() {
            Port::Constant(value) => {
                64 - ((value - 1).leading_zeros() as usize)
            }
            _ => 64
        };
        self.allocs
            .push((for_.variant(), Cell::Register(index_reg_bits)));
        Action::None
    }

    fn start_enable(&mut self, enable: &mut Ir, _pool: &mut P) -> Action {
        if let Port::Variable(var) = *enable.kill() {
            self.allocs.push((var, Cell::Register(64)));
        }
        Action::None
    }

    fn finish_component(&mut self, comp: &mut Component, pool: &mut P) {
        for (var, cell) in mem::take(&mut self.allocs) {
            comp.cell_alloc.insert(var, pool.add(cell));
        }
    }
}
