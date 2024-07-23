//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use canonicalize::Canonicalize;
use collapse_control::CollapseControl;
use copy_prop::CopyProp;
use dead_code::DeadCode;

use crate::{
    component::Component, from_ast::AsGeneratorPool, visitor::Visitor
};

pub mod canonicalize;
pub mod cell_alloc;
pub mod collapse_control;
pub mod copy_prop;
pub mod dead_code;

pub struct PassRunner<P: AsGeneratorPool> {
    passes: Vec<Box<dyn Visitor<P>>>
}

impl<P: AsGeneratorPool> PassRunner<P> {
    /// This pass runner has no registered passes. Use [`Runner::default`] for
    /// default passes to be already registered.
    pub fn empty() -> Self {
        Self { passes: Vec::new() }
    }

    pub fn register<V: Visitor<P> + 'static>(&mut self, pass: V) {
        self.passes.push(Box::new(pass));
    }

    pub fn run(&mut self, comp: &mut Component, pool: &mut P) {
        for pass in &mut self.passes {
            pass.traverse_component(comp, pool)
        }
    }
}

impl<P: AsGeneratorPool> Default for PassRunner<P> {
    fn default() -> Self {
        let mut runner = Self::empty();
        runner.register(Canonicalize);
        runner.register(CopyProp);
        runner.register(DeadCode::default());
        runner.register(CollapseControl);
        runner
    }
}
