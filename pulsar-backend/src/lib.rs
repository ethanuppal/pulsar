//! The pulsar backend is currently under construction. The goal is for a
//! modular but expressive (in e.g. output file location) interface. A
//! [`calyx_backend::CalyxBackend`] is under construction.
//!
//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use pulsar_ir::from_ast::GeneratedTopLevel;
use std::path::PathBuf;

pub mod calyx;

// This interface hasn't been finalized yet, so it is quite sloppy as written

pub enum Output {
    Stdout,
    Stderr,
    File(PathBuf)
}

pub trait PulsarBackend {
    type InitInput;
    type Error;

    /// Initializes the backend.
    fn new(input: Self::InitInput) -> Self;

    /// Consumes the backend and produces an output.
    fn run(
        self, code: Vec<GeneratedTopLevel>, output: Output
    ) -> Result<(), Self::Error>;
}
