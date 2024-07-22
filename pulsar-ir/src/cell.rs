//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use std::fmt::{self, Display};

use crate::memory::Memory;
use pulsar_frontend::ast::ty::{LiquidTypeValue, Type, TypeValue};

/// A hardware element capable of storage.
pub enum Cell {
    Memory(Memory),

    /// `Cell::Register(bit_width)` is a register `bit_width` bits wide.
    Register(usize)
}

impl Cell {
    pub fn from(ty: &Type) -> Self {
        match ty.value {
            TypeValue::Unit => Self::Register(0),
            TypeValue::Var(_) => panic!(),
            TypeValue::Name(_) => todo!(),
            TypeValue::Int64 => Self::Register(64),
            TypeValue::Array(element_type, element_count) => {
                let LiquidTypeValue::Equal(length) = element_count.value else {
                    panic!("liquid type not resolved");
                };
                Self::Memory(Memory::new(length, element_type.size(), 1))
            }
            TypeValue::Function {
                inputs: _,
                outputs: _
            } => panic!()
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Memory(memory) => {
                write!(
                    f,
                    "Memory(length={}, element={}, bank={})",
                    memory.length, memory.element, memory.bank
                )
            }
            Cell::Register(bit_width) => {
                write!(f, "Register(width={})", bit_width)
            }
        }
    }
}
