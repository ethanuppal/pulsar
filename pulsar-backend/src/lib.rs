//! The pulsar backend is currently under construction.
//!
//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use pulsar_ir::{component::Component, from_ast::AsGeneratorPool};
use pulsar_utils::id::Gen;
use target::{OutputFile, Target};
use transform::Transform;

pub mod target;
pub mod transform;

pub struct Backend<P: AsGeneratorPool> {
    target: Option<Box<dyn Target<P>>>,
    transforms: Vec<Box<dyn Transform<P>>>
}

impl<P: AsGeneratorPool> Backend<P> {
    pub fn emit(
        &mut self, comp: &Component, pool: &mut P, gen: &mut Gen,
        output: OutputFile
    ) -> anyhow::Result<()> {
        let mut transforms = self.transforms.iter_mut();
        let mut result_comp;
        let comp = if let Some(first) = transforms.next() {
            result_comp = first.apply(comp, pool, gen)?;
            for transform in transforms {
                result_comp = transform.apply(&result_comp, pool, gen)?;
            }
            &result_comp
        } else {
            comp
        };
        self.target
            .as_mut()
            .expect("no target specified")
            .emit(comp, pool, output)
    }
}

pub struct BackendBuilder<P: AsGeneratorPool> {
    backend: Backend<P>
}

impl<P: AsGeneratorPool> BackendBuilder<P> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn target<T: Target<P> + 'static>(mut self, target: T) -> Self {
        self.backend.target = Some(Box::new(target));
        self
    }

    pub fn through<T: Transform<P> + 'static>(mut self, transform: T) -> Self {
        self.backend.transforms.push(Box::new(transform));
        self
    }

    pub fn build(self) -> Backend<P> {
        self.backend
    }
}

impl<P: AsGeneratorPool> Default for BackendBuilder<P> {
    fn default() -> Self {
        Self {
            backend: Backend {
                target: None,
                transforms: Vec::new()
            }
        }
    }
}
