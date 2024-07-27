//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use pulsar_utils::pool::Handle;

use super::variable::Variable;
use std::fmt::{self, Display};

/// A port represents a constant or an lvalue.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Port {
    Constant(i64),
    Variable(Variable),
    /// Non-canonical -- the `Canonicalize` collapses these into
    /// [`Port::Access`].
    PartialAccess(Handle<Port>, Handle<Port>),
    Access(Variable, Vec<Handle<Port>>)
}

impl Port {
    pub fn root_var(&self) -> Option<Variable> {
        match self {
            Port::Variable(var) | Port::Access(var, _) => Some(*var),
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
            )
        }
    }
}
