//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use self::{label::Name, operand::Operand, variable::Variable};
use std::fmt::{self, Display};

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
    Assign(Variable, Operand),
    GetParam(Variable),
    Return(Option<Operand>),

    /// `LocalAlloc(result, size, count)` allocates an array of `count`
    /// elements, each of `size` bytes, and stores a pointer to the array in
    /// `result`.
    LocalAlloc(Variable, usize, usize),

    /// `Store { result, value, index }` loads `value` into index `index` of
    /// `result`.
    Store {
        result: Variable,
        value: Operand,
        index: Operand
    },

    /// `Load { result, value, index }` loads the value at index `index` of
    /// `value` into `result`.
    Load {
        result: Variable,
        value: Operand,
        index: Operand
    },

    Map {
        result: Variable,
        parallel_factor: usize,
        f: Name,
        input: Operand,
        length: usize
    },
    Call(Option<Variable>, Name, Vec<Operand>)
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
            Self::Assign(result, from) => write!(f, "{} = {}", result, from),
            Self::GetParam(result) => write!(f, "{} = <next param>", result),
            Self::Return(value_opt) => write!(
                f,
                "ret{}",
                if let Some(value) = value_opt {
                    format!(" {}", value)
                } else {
                    "".into()
                }
            ),
            Self::LocalAlloc(result, size, count) => {
                write!(f, "{} = <{} * ({} bytes)>", result, count, size)
            }
            Self::Store {
                result,
                value,
                index
            } => {
                write!(f, "{}[{}] = {}", result, index, value)
            }
            Self::Load {
                result,
                value,
                index
            } => {
                write!(f, "{} = {}[{}]", result, index, value)
            }
            Self::Map {
                result,
                parallel_factor,
                f: func,
                input,
                length: _
            } => {
                write!(
                    f,
                    "{} = map<{}>({}, {})",
                    result, parallel_factor, func, input
                )
            }
            Self::Call(result_opt, name, args) => {
                write!(
                    f,
                    "{}{}({})",
                    if let Some(result) = result_opt {
                        format!("{} = ", result)
                    } else {
                        "".into()
                    },
                    name,
                    args.iter()
                        .map(|arg| arg.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }
}
