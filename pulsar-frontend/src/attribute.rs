// Copyright (C) 2024 Ethan Uppal. All rights reserved.

#[repr(u8)]
pub enum Attribute {
    /// Not present in user source.
    Generated
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub struct Attributes {
    bitmap: u16
}

impl Attributes {
    pub fn add(&mut self, attr: Attribute) {
        self.bitmap |= 1 << (attr as u8);
    }

    pub fn has(&self, attr: Attribute) -> bool {
        self.bitmap & (1 << (attr as u8)) != 0
    }
}
