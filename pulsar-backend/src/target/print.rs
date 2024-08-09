// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

use super::{OutputFile, Target};
use pulsar_ir::{component::Component, from_ast::AsGeneratorPool};
use std::{
    fs,
    io::{self, Write}
};

pub struct PrintTarget;

impl<P: AsGeneratorPool> Target<P> for PrintTarget {
    fn emit(
        &mut self, comp: &Component, _pool: &P, output: OutputFile
    ) -> anyhow::Result<()> {
        match output {
            OutputFile::Stdout => writeln!(&mut io::stdout(), "{}", comp),
            OutputFile::Stderr => writeln!(&mut io::stderr(), "{}", comp),
            OutputFile::File(path) => {
                let mut file = fs::OpenOptions::new().write(true).open(path)?;
                writeln!(file, "{}", comp)
            }
        }?;
        Ok(())
    }
}
