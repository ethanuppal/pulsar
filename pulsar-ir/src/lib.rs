//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use self::{port::Port, variable::Variable};
use port::PortUsage;
use pulsar_utils::pool::Handle;
use std::fmt::{self, Display};

pub mod analysis;
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
    /// A combinational addition
    Add(Handle<Port>, Handle<Port>, Handle<Port>),
    /// A pipelined multiply with an II of 1 and latency of 4
    Mul(Handle<Port>, Handle<Port>, Handle<Port>),
    Assign(Handle<Port>, Handle<Port>)
}

impl Ir {
    pub fn kill(&self) -> Handle<Port> {
        match self {
            Ir::Add(lhs, _, _) | Ir::Mul(lhs, _, _) | Ir::Assign(lhs, _) => *lhs
        }
    }

    pub fn kill_var(&self) -> Option<Variable> {
        self.kill().root_var()
    }

    // messy

    /// The top-level ports read by this IR instruction.
    pub fn gen_ref(&self) -> Vec<Handle<Port>> {
        match self {
            Ir::Add(_, port, port2) | Ir::Mul(_, port, port2) => {
                vec![*port, *port2]
            }
            Ir::Assign(_, port) => {
                vec![*port]
            }
        }
    }

    /// The top-level ports read by this IR instruction.
    pub fn gen_mut(&mut self) -> Vec<&mut Handle<Port>> {
        match self {
            Ir::Add(_, port, port2) | Ir::Mul(_, port, port2) => {
                vec![port, port2]
            }
            Ir::Assign(_, port) => {
                vec![port]
            }
        }
    }

    /// Every single port read by this IR instruction, recursively.
    pub fn gen_used(&self) -> Vec<Handle<Port>> {
        use port::PortUsage;

        match self {
            Ir::Add(_, port, port2) | Ir::Mul(_, port, port2) => {
                let mut result = port.ports_used();
                result.extend(port2.ports_used());
                result
            }
            Ir::Assign(_, port) => port.ports_used()
        }
    }

    /// The top-level ports in this IR instruction.
    pub fn ports_ref(&self) -> Vec<Handle<Port>> {
        match self {
            Ir::Add(port0, port1, port2) | Ir::Mul(port0, port1, port2) => {
                vec![*port0, *port1, *port2]
            }
            Ir::Assign(port0, port1) => {
                vec![*port0, *port1]
            }
        }
    }

    /// The top-level ports in this IR instruction.
    pub fn ports_mut(&mut self) -> Vec<&mut Handle<Port>> {
        match self {
            Ir::Add(port0, port1, port2) | Ir::Mul(port0, port1, port2) => {
                vec![port0, port1, port2]
            }
            Ir::Assign(port0, port1) => {
                vec![port0, port1]
            }
        }
    }

    /// Every single port referenced in this IR instruction.
    pub fn ports_used_ref(&self) -> Vec<Handle<Port>> {
        match self {
            Ir::Add(port0, port1, port2) | Ir::Mul(port0, port1, port2) => {
                vec![port0.ports_used(), port1.ports_used(), port2.ports_used()]
            }
            Ir::Assign(port0, port1) => {
                vec![port0.ports_used(), port1.ports_used()]
            }
        }
        .iter()
        .flatten()
        .cloned()
        .collect()
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
