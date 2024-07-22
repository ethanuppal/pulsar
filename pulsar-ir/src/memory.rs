//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

/// todo: everything about this, including like multilayering and stuff
pub struct Memory {
    pub length: usize,
    pub element: usize,
    pub bank: usize
}

impl Memory {
    pub fn new(length: usize, element: usize, bank: usize) -> Self {
        Self {
            length,
            element,
            bank
        }
    }
}
