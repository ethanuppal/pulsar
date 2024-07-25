//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::memory::{Memory, MemoryLevel};
use pulsar_frontend::ast::ty::{LiquidTypeValue, Type, TypeValue};
use std::fmt::{self, Display};

/// A hardware element capable of storage.
pub enum Cell {
    Memory(Memory),

    /// `Cell::Register(bit_width)` is a register `bit_width` bits wide.
    Register(usize)
}

pub const BITS_PER_BYTE: usize = 8;

impl Cell {
    pub fn from(ty: &Type) -> Self {
        match ty.value {
            TypeValue::Unit => Self::Register(0),
            TypeValue::Var(..) => panic!(),
            TypeValue::Name(_) => todo!(),
            TypeValue::Int64 => Self::Register(64),
            TypeValue::Array(element_type, element_count) => {
                let LiquidTypeValue::Equal(length) = element_count.value else {
                    panic!("liquid type not resolved");
                };
                let mut levels = vec![MemoryLevel { length, bank: 1 }];
                let element = match Cell::from(&element_type) {
                    Cell::Memory(sub_memory) => {
                        levels.extend(sub_memory.levels().iter().cloned());
                        sub_memory.element()
                    }
                    Cell::Register(element) => element
                };
                Self::Memory(Memory::from(levels, element))
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
                for level in memory.levels() {
                    write!(f, "[{} bank {}]", level.length, level.bank)?;
                }
                Cell::Register(memory.element()).fmt(f)
            }
            Cell::Register(bit_width) => {
                write!(f, "[0:{}]()", bit_width - 1)
            }
        }
    }
}
