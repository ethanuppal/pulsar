// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

pub type Id = usize;

#[derive(Default)]
pub struct Gen {
    next: Id
}

impl Gen {
    pub fn new() -> Self {
        Self::default()
    }

    // Clippy wants me to implement [`Iterator`] or rename this from `next`.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Id {
        let result = self.next;
        self.next += 1;
        result
    }
}
