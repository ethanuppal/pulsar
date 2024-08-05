//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::variable::Variable;
use pulsar_utils::pool::Handle;
use std::{
    fmt::{self, Display},
    ops::Deref
};

/// A port represents a constant or an lvalue.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Port {
    Constant(i64),
    Variable(Variable),
    /// Non-canonical -- the `Canonicalize` collapses these into
    /// [`Port::Access`].
    PartialAccess(Handle<Port>, Handle<Port>),
    Access(Variable, Vec<Handle<Port>>),
    LoweredAccess(Variable)
}

impl Port {
    pub fn root_var(&self) -> Option<Variable> {
        match self {
            Port::Variable(var)
            | Port::Access(var, _)
            | Port::LoweredAccess(var) => Some(*var),
            Port::PartialAccess(port, _) => port.root_var(),
            _ => None
        }
    }

    /// The variables this port references.
    pub fn vars(&self) -> Vec<Variable> {
        match self {
            Port::Variable(var) => vec![*var],
            Port::Access(array_var, indices) => {
                let mut result = vec![*array_var];
                for index in indices {
                    result.extend(index.vars());
                }
                result
            }
            Port::PartialAccess(array, index) => {
                let mut result = array.vars();
                result.extend(index.vars());
                result
            }
            Self::LoweredAccess(var) => vec![*var],
            _ => vec![]
        }
    }
}

impl Display for Port {
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
            ),
            Self::LoweredAccess(var) => write!(f, "{}[<generated>]", var)
        }
    }
}

// messy

pub trait PortUsage {
    fn ports_used(&self) -> Vec<Handle<Port>>;
}

impl PortUsage for Handle<Port> {
    fn ports_used(&self) -> Vec<Handle<Port>> {
        let mut result = vec![*self];
        match self.deref() {
            Port::Constant(_) | Port::Variable(_) | Port::LoweredAccess(_) => {}
            Port::PartialAccess(_, port) => result.push(*port),
            Port::Access(_, ports) => result.extend(ports)
        }
        result
    }
}
