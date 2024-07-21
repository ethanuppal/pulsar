//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::memory::Memory;
use pulsar_frontend::ast::ty::{Type, TypeValue};

/// A hardware element capable of storage.
pub enum Cell {
    Memory(Memory),

    /// `Cell::Register(bit_width)` is a register `bit_width` bits wide.
    Register(usize)
}

impl From<Type> for Cell {
    fn from(ty: Type) -> Self {
        match &ty.value {
            TypeValue::Unit => Self::Register(0),
            TypeValue::Var(_) => panic!(),
            TypeValue::Name(_) => todo!(),
            TypeValue::Int64 => Self::Register(64),
            TypeValue::Array(element_type, element_count) => {
                todo!("Memory from {}/{}", element_type, element_count)
            }
            TypeValue::Function { inputs, outputs } => panic!()
        }
    }
}
