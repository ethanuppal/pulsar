//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::pass::cell_alloc::min_bits_to_represent;

/// Banking is currently completely ignored.
#[derive(Clone)]
pub struct MemoryLevel {
    pub length: usize,
    pub bank: usize
}

#[derive(Clone)]
pub struct Memory {
    /// The first level is the outermost "array", and so on.
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

    pub fn from(levels: Vec<MemoryLevel>, element: usize) -> Self {
        Self { levels, element }
    }

    pub fn levels(&self) -> &[MemoryLevel] {
        &self.levels
    }

    pub fn element(&self) -> usize {
        self.element
    }

    pub fn flattened_length(&self) -> usize {
        self.levels.iter().map(|level| level.length).product()
    }

    pub fn flattened_address_width(&self) -> usize {
        min_bits_to_represent(self.flattened_length())
    }
}
