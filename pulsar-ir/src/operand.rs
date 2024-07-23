//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::variable::Variable;
use std::fmt::{self, Display};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Operand {
    Constant(i64),
    Variable(Variable),
    /// Non-canonical
    PartialAccess(Box<Operand>, Box<Operand>),
    Access(Variable, Vec<Operand>)
}

impl Operand {
    /// Assumes that
    pub fn gen_var(&self) -> Option<Variable> {
        match self {
            Operand::Variable(var) | Operand::Access(var, _) => Some(*var),
            Operand::PartialAccess(operand, _) => operand.gen_var(),
            _ => None
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Constant(value) => value.fmt(f),
            Self::Variable(var) => var.fmt(f),
            Self::PartialAccess(mem, index) => write!(f, "{}[{}]", mem, index),
            Self::Access(mem, indices) => write!(
                f,
                "{}{}",
                mem,
                indices
                    .iter()
                    .map(|i| format!("[{}]", i))
                    .collect::<Vec<_>>()
                    .join("")
            )
        }
    }
}
