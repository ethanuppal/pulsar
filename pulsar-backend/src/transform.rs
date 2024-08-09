//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use pulsar_ir::{component::Component, from_ast::AsGeneratorPool};
use pulsar_utils::id::Gen;

pub mod agen;

pub trait Transform<P: AsGeneratorPool> {
    fn apply(
        &mut self, comp: &Component, pool: &mut P, gen: &mut Gen
    ) -> anyhow::Result<Component>;
}
