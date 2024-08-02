//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use std::ops::BitOr;

use crate::{
    component::Component, from_ast::AsGeneratorPool, visitor::VisitorMut,
};
use calculate_timing::CalculateTiming;
use canonicalize::Canonicalize;
use cell_alloc::CellAlloc;
use collapse_control::CollapseControl;
use copy_prop::CopyProp;
use dead_code::DeadCode;
use rewrite_accesses::RewriteAccesses;
use well_formed::WellFormed;

pub mod calculate_timing;
pub mod canonicalize;
pub mod cell_alloc;
pub mod collapse_control;
pub mod copy_prop;
pub mod dead_code;
pub mod rewrite_accesses;
pub mod well_formed;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PassOptions(u64);

impl PassOptions {
    pub const NONE: Self = PassOptions(0);
    pub const PRESERVE_TIMING: Self = PassOptions(1 << 0);

    pub fn contains(&self, options: PassOptions) -> bool {
        (self.0 & options.0) == options.0
    }
}

impl BitOr for PassOptions {
    type Output = PassOptions;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

pub trait Pass<P: AsGeneratorPool>: VisitorMut<P> {
    fn name(&self) -> &str;

    fn setup(&mut self, options: PassOptions);
}

enum PassOp<P: AsGeneratorPool> {
    Boxed(Box<dyn Pass<P>>, PassOptions),
    Coverge(usize),
    EndConverge,
}

pub struct PassRunner<P: AsGeneratorPool> {
    passes: Vec<PassOp<P>>,
}

impl<P: AsGeneratorPool> PassRunner<P> {
    /// The minimal pass runner permitted.
    pub fn core() -> Self {
        let mut runner = Self { passes: Vec::new() };
        runner.register(WellFormed::default(), PassOptions::NONE);
        runner.register(Canonicalize, PassOptions::NONE);
        runner
    }

    /// The pass runner used by [`from_ast::ast_to_ir`]; notably, it does not preserve timing in
    /// control collasing.
    pub fn compile() -> Self {
        let mut runner = Self::core();
        runner.register_converge(10, |runner| {
            runner.register(CopyProp, PassOptions::NONE);
            runner.register(CalculateTiming, PassOptions::NONE);
            // runner.register(DeadCode::new(true));
        });
        runner.register(CollapseControl::default(), PassOptions::NONE);
        runner.register(CellAlloc, PassOptions::NONE);
        runner.register(CalculateTiming, PassOptions::NONE);
        runner
    }

    /// A pass runner for lowering a [`Component`] for an emission target, preserving timing.
    pub fn lower() -> Self {
        let mut runner = Self::core();
        runner.register(RewriteAccesses, PassOptions::PRESERVE_TIMING);
        runner.register_converge(10, |runner| {
            runner.register(CopyProp, PassOptions::PRESERVE_TIMING);
            // runner.register(DeadCode::new(false));
        });
        runner
            .register(CollapseControl::default(), PassOptions::PRESERVE_TIMING);
        runner
    }

    pub fn register<V: Pass<P> + 'static>(
        &mut self, pass: V, options: PassOptions,
    ) {
        self.passes.push(PassOp::Boxed(Box::new(pass), options));
    }

    pub fn register_converge<F: FnOnce(&mut Self)>(
        &mut self, iter_limit: usize, f: F,
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
                PassOp::Boxed(pass, options) => {
                    log::info!(
                        "{}running pass '{}'",
                        if in_convergence { "  " } else { "" },
                        pass.name()
                    );
                    if in_convergence {
                        convergence_region.push((pass, *options));
                    } else {
                        pass.setup(*options);
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
                            for (pass, options) in &mut convergence_region {
                                pass.setup(*options);
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
