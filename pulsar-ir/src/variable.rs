//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use pulsar_utils::id::Id;
use std::fmt::{self, Display};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Variable {
    id: Id
}

impl From<Id> for Variable {
    fn from(value: Id) -> Self {
        Self { id: value }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i{}", self.id)
    }
}