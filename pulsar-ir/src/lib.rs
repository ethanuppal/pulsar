//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use self::{port::Port, variable::Variable};
use pulsar_utils::pool::Handle;
use std::fmt::{self, Display};

pub mod cell;
pub mod component;
pub mod control;
pub mod from_ast;
pub mod label;
pub mod memory;
pub mod pass;
pub mod port;
pub mod variable;
pub mod visitor;

#[derive(Clone)]
pub enum Ir {
    Add(Handle<Port>, Handle<Port>, Handle<Port>),
    Mul(Handle<Port>, Handle<Port>, Handle<Port>),
    Assign(Handle<Port>, Handle<Port>)
}

impl Ir {
    pub fn kill(&self) -> Handle<Port> {
        match self {
            Ir::Add(lhs, _, _) | Ir::Mul(lhs, _, _) | Ir::Assign(lhs, _) => *lhs
        }
    }

    pub fn gen(&self) -> Vec<Handle<Port>> {
        match self {
            Ir::Add(_, port, port2) | Ir::Mul(_, port, port2) => {
                vec![*port, *port2]
            }
            Ir::Assign(_, port) => {
                vec![*port]
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

impl From<Variable> for Port {
    fn from(value: Variable) -> Self {
        Self::Variable(value)
    }
}
