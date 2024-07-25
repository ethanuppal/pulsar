//! The pulsar backend is currently under construction.
//!
//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use pulsar_ir::component::Component;
use target::{OutputFile, Target};
use transform::Transform;

pub mod target;
pub mod transform;

// pub trait EmissionTarget {
//     type InitInput;
//     type Error;

//     /// Initializes the backend.
//     fn new(input: Self::InitInput) -> Self;

//     /// Consumes the backend and produces an output.
//     fn run(
//         self, code: Vec<GeneratedTopLevel>, output: Output
//     ) -> Result<(), Self::Error>;
// }

#[derive(Default)]
pub struct Backend {
    target: Option<Box<dyn Target>>,
    transforms: Vec<Box<dyn Transform>>
}

impl Backend {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn target<T: Target + 'static>(mut self, target: T) -> Self {
        self.target = Some(Box::new(target));
        self
    }

    pub fn through<T: Transform + 'static>(mut self, transform: T) -> Self {
        self.transforms.push(Box::new(transform));
        self
    }

    pub fn lower(
        &mut self, mut comp: Component, output: OutputFile
    ) -> anyhow::Result<()> {
        for transform in &mut self.transforms {
            comp = transform.apply(comp)?;
        }
        self.target
            .as_mut()
            .expect("no target specified")
            .emit(comp, output)
    }
}
