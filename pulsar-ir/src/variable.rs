// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
use pulsar_utils::id::{Gen, Id};
use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Variable {
    id: Id
}

impl Variable {
    pub fn new() -> Self {
        Self {
            id: Gen::next("IR variable")
        }
    }
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "i{}", self.id)
    }
}
