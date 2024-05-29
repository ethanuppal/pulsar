// Copyright (C) 2024 Ethan Uppal. All rights reserved.
use super::variable::Variable;
use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Operand {
    Constant(i64),
    Variable(Variable)
}

impl Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Constant(value) => value.fmt(f),
            Self::Variable(var) => var.fmt(f)
        }
    }
}
