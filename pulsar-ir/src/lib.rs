//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use self::{label::Name, operand::Operand, variable::Variable};
use std::fmt::{self, Display};

pub mod cell;
pub mod component;
pub mod control;
pub mod from_ast;
pub mod label;
pub mod memory;
pub mod operand;
pub mod variable;

pub enum Ir {
    Add(Variable, Operand, Operand),
    Mul(Variable, Operand, Operand),
    Assign(Variable, Operand)
}

impl Display for Ir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Add(result, lhs, rhs) => {
                write!(f, "{} = {} + {}", result, lhs, rhs)
            }
            Self::Mul(result, lhs, rhs) => {
                write!(f, "{} = {} * {}", result, lhs, rhs)
            }
            Self::Assign(result, from) => write!(f, "{} = {}", result, from)
        }
    }
}
