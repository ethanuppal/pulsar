//! This crate defines utilities for the pulsar compiler, such as error
//! reporting and semantically-rich locations. It also implements data
//! structures such as a directed graph ([`digraph::Digraph`]) and disjoint sets
//! ([`disjoint_set::DisjointSets`]).
//!
//! Copyright (C) 2024 Ethan Uppal. All rights reserved.

use std::ops::Deref;

pub mod digraph;
pub mod disjoint_set;
pub mod environment;
pub mod error;
pub mod format;
pub mod id;
pub mod loc;
pub mod mutcell;

/// A type whose `clone()` involves copying no more than 8-16 bytes of data.
pub trait CheapClone: Clone {}
impl<T: Clone + Deref> CheapClone for T {}
