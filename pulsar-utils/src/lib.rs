//! This crate defines utilities for the pulsar compiler, such as error
//! reporting and semantically-rich locations. It also implements data
//! structures such as a scoped map ([`environment::Environment`]) and
//! union-find ([`disjoint_sets::DisjointSets`]).
//!
//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

pub mod disjoint_sets;
pub mod environment;
pub mod error;
pub mod format;
pub mod id;
pub mod pool;
pub mod rrc;
pub mod span;
