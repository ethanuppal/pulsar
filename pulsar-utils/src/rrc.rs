//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use std::{cell::RefCell, rc::Rc};

/// An alias for [`Rc<RefCell<T>>`].
pub type RRC<T> = Rc<RefCell<T>>;

/// A convenience constructor for [`RRC<T>`].
pub fn rrc<T>(value: T) -> RRC<T> {
    Rc::new(RefCell::new(value))
}
