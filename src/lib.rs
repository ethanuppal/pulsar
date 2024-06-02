//! Pulsar is a high-level programming language for building hardware
//! accelerators. This crate (the `pulsar` crate) contains a driver that links
//! together all the langauge components. The documentation for each individual
//! component is linked under the "Re-exports" section.
//!
//! Copyright (C) 2024 Ethan Uppal. All rights reserved.

#[doc(no_inline)]
pub use pulsar_backend;
#[doc(no_inline)]
pub use pulsar_frontend;
#[doc(no_inline)]
pub use pulsar_ir;
#[doc(no_inline)]
pub use pulsar_utils;
