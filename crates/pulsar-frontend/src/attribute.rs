// Copyright (C) 2024 Ethan Uppal. All rights reserved.

#[repr(usize)]
pub enum Attribute {
    Generated
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub struct Attributes {
    bitmap: u16
}

impl Attributes {
    pub fn add(&mut self, attr: Attribute) {
        self.bitmap |= 1 << (attr as usize);
    }

    pub fn has(&self, attr: Attribute) -> bool {
        self.bitmap & (1 << (attr as usize)) != 0
    }
}
