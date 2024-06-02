//! The pulsar backend is currently under construction. The goal is for a
//! modular but expressive (in e.g. output file location) interface. A
//! [`calyx_backend::CalyxBackend`] is under construction.
//!
//! Copyright (C) 2024 Ethan Uppal. All rights reserved.

use pulsar_ir::generator::GeneratedTopLevel;

pub mod calyx_backend;

// This interface hasn't been finalized yet, so it is quite sloppy as written

pub trait PulsarBackend {
    type ExtraInput;
    type Error;

    fn new() -> Self;
    fn run(
        &mut self, code: Vec<GeneratedTopLevel>, input: Self::ExtraInput
    ) -> Result<(), Self::Error>;
}
