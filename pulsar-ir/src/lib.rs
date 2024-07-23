//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use pulsar_frontend::op::Op;

use self::{operand::Operand, variable::Variable};
use std::{
    convert,
    fmt::{self, Display}
};

pub mod cell;
pub mod component;
pub mod control;
pub mod from_ast;
pub mod label;
pub mod memory;
pub mod operand;
pub mod pass;
pub mod variable;
pub mod visitor;

pub enum Ir {
    Add(Variable, Operand, Operand),
    Mul(Variable, Operand, Operand),
    Assign(Operand, Operand)
}

impl Ir {
    pub fn assign<R: Into<Operand>>(result: R, operand: Operand) -> Self {
        Self::Assign(result.into(), operand)
    }

    pub fn kill(&self) -> Operand {
        match self {
            Ir::Add(lhs, _, _) | Ir::Mul(lhs, _, _) => Operand::from(*lhs),
            Ir::Assign(lhs, _) => lhs.clone()
        }
    }

    pub fn gen(&self) -> Vec<&Operand> {
        match self {
            Ir::Add(_, operand, operand2) | Ir::Mul(_, operand, operand2) => {
                vec![operand, operand2]
            }
            Ir::Assign(_attr, operand) => {
                vec![operand]
            }
        }
    }
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

impl From<Variable> for Operand {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}
