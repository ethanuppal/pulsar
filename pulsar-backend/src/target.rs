//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use pulsar_ir::{component::Component, from_ast::AsGeneratorPool};
use std::path::PathBuf;

pub mod calyx;
pub mod print;

/// This interface hasn't been finalized yet, so it is quite sloppy as written
pub enum OutputFile {
    Stdout,
    Stderr,
    File(PathBuf)
}

pub trait Target<P: AsGeneratorPool> {
    fn emit(
        &mut self, _comp: &Component, pool: &P, output: OutputFile
    ) -> anyhow::Result<()>;
}
