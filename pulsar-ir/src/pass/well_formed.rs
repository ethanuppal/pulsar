//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    from_ast::AsGeneratorPool,
    port::Port,
    visitor::{Action, Visitor},
    Ir
};

/// This pass and `Canonicalize` after it are applied before every other pass.
pub struct WellFormed;

impl<P: AsGeneratorPool> Visitor<P> for WellFormed {
    fn start_enable(&mut self, enable: &mut Ir, _pool: &mut P) -> Action {
        if let Port::Constant(_) = &*enable.kill() {
            panic!(
                "port {} on lhs of ir operation is not an lvalue",
                enable.kill()
            );
        }
        Action::None
    }
}
