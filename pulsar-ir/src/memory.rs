//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

pub struct MemoryLevel {
    pub length: usize,
    pub bank: usize
}

pub struct Memory {
    levels: Vec<MemoryLevel>,
    element: usize
}

// how to handle multidimensional memories with banking per level and other
// stuff?

impl Memory {
    pub fn new(length: usize, element: usize, bank: usize) -> Self {
        Self {
            levels: vec![MemoryLevel { length, bank }],
            element
        }
    }

    pub fn levels(&self) -> &[MemoryLevel] {
        &self.levels
    }

    pub fn element(&self) -> usize {
        self.element
    }
}
