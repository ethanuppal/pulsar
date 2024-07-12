//! Pulsar is a high-level programming language for building hardware
//! accelerators. This crate (the `pulsar` crate) contains a driver that links
//! together all the langauge components. The documentation for each individual
//! component is linked under the "Re-exports" section.
//!
//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

#[doc(no_inline)]
pub use calyx_builder;
#[doc(no_inline)]
pub use pulsar_backend;
#[doc(no_inline)]
pub use pulsar_frontend;
#[doc(no_inline)]
pub use pulsar_ir;
#[doc(no_inline)]
pub use pulsar_utils;
