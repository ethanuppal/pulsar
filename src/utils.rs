//! This crate defines utilities for the pulsar compiler, such as error
//! reporting and semantically-rich locations.

use std::ops::Deref;

pub mod context;
pub mod digraph;
pub mod disjoint_set;
pub mod error;
pub mod format;
pub mod id;
pub mod loc;
pub mod mutcell;

/// A type whose `clone()` involves copying no more than 8-16 bytes of data.
pub trait CheapClone: Clone {}
impl<T: Clone + Deref> CheapClone for T {}
