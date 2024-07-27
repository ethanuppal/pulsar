//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    component::Component, from_ast::AsGeneratorPool, visitor::VisitorMut
};
use canonicalize::Canonicalize;
use cell_alloc::CellAlloc;
use collapse_control::CollapseControl;
use copy_prop::CopyProp;
use dead_code::DeadCode;
use well_formed::WellFormed;

pub mod canonicalize;
pub mod cell_alloc;
pub mod collapse_control;
pub mod copy_prop;
pub mod dead_code;
pub mod well_formed;

pub trait Pass<P: AsGeneratorPool>: VisitorMut<P> {
    fn name(&self) -> &str;
}

enum PassOp<P: AsGeneratorPool> {
    Boxed(Box<dyn Pass<P>>),
    Coverge(usize),
    EndConverge
}

pub struct PassRunner<P: AsGeneratorPool> {
    passes: Vec<PassOp<P>>
}

impl<P: AsGeneratorPool> PassRunner<P> {
    /// The minimal pass runner permitted.
    pub fn core() -> Self {
        let mut runner = Self { passes: Vec::new() };
        runner.register(WellFormed::default());
        runner.register(Canonicalize);
        runner
    }

    pub fn register<V: Pass<P> + 'static>(&mut self, pass: V) {
        self.passes.push(PassOp::Boxed(Box::new(pass)));
    }

    pub fn register_converge<F: FnOnce(&mut Self)>(
        &mut self, iter_limit: usize, f: F
    ) {
        self.passes.push(PassOp::Coverge(iter_limit));
        f(self);
        self.passes.push(PassOp::EndConverge);
    }

    pub fn run(&mut self, comp: &mut Component, pool: &mut P) {
        let mut in_convergence = false;
        let mut convergence_iter_limit = 0;
        let mut convergence_region = Vec::new();

        for pass_op in &mut self.passes {
            match pass_op {
                PassOp::Boxed(pass) => {
                    log::info!(
                        "{}running pass '{}'",
                        if in_convergence { "  " } else { "" },
                        pass.name()
                    );
                    if in_convergence {
                        convergence_region.push(pass);
                    } else {
                        pass.traverse_component(comp, pool);
                    }
                }
                PassOp::Coverge(iter_limit) => {
                    log::info!("begin pass convergence region");
                    in_convergence = true;
                    convergence_iter_limit = *iter_limit;
                }
                PassOp::EndConverge => {
                    log::info!("end pass convergence region");
                    if in_convergence {
                        loop {
                            let mut did_modify = false;
                            for pass in &mut convergence_region {
                                did_modify |=
                                    pass.traverse_component(comp, pool);
                            }
                            if !did_modify || convergence_iter_limit == 0 {
                                break;
                            }
                            convergence_iter_limit -= 1;
                        }
                        in_convergence = false;
                    }
                }
            };
        }
    }
}

impl<P: AsGeneratorPool> Default for PassRunner<P> {
    fn default() -> Self {
        let mut runner = Self::core();
        runner.register_converge(10, |runner| {
            runner.register(CopyProp);
            runner.register(DeadCode::default());
        });
        runner.register(CollapseControl);
        runner.register(CellAlloc);
        runner
    }
}
