//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

type Repr = u16;

#[repr(u8)]
pub enum Attribute {
    /// Not present in user source.
    Generated = 1 << 0,
    /// A main accelerator kernel for which an address generator should be
    /// created.
    Kernel = 1 << 1
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub struct Attributes {
    bitmap: Repr
}

impl Attributes {
    pub fn add(&mut self, attr: Attribute) {
        self.bitmap |= 1 << (attr as u8);
    }

    pub fn with(mut self, attr: Attribute) -> Self {
        self.add(attr);
        self
    }

    pub fn has(&self, attr: Attribute) -> bool {
        self.bitmap & (1 << (attr as u8)) != 0
    }
}

impl<T: IntoIterator<Item = Attribute>> From<T> for Attributes {
    fn from(value: T) -> Self {
        let mut result = Attributes::default();
        for attr in value {
            result.add(attr);
        }
        result
    }
}

pub trait AttributeProvider {
    fn attributes_ref(&self) -> &Attributes;
    fn attributes_mut(&mut self) -> &mut Attributes;

    fn has_attribute(&self, attr: Attribute) -> bool {
        self.attributes_ref().has(attr)
    }

    fn add_attribute(&mut self, attr: Attribute) {
        self.attributes_mut().add(attr);
    }
}
