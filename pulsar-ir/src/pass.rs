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
use rewrite_accesses::RewriteAccesses;
use std::ops::BitOr;
use well_formed::WellFormed;

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
    fn from(options: PassOptions, _comp: &mut Component, pool: &mut P) -> Self;

    fn name() -> &'static str;
}

type PassCall<P> = Box<dyn FnMut(&mut Component, &mut P, PassOptions) -> bool>;

enum PassOp<P: AsGeneratorPool> {
    Call(PassCall<P>, &'static str, PassOptions),
    Coverge(usize),
    EndConverge
}

pub struct PassRunner<P: AsGeneratorPool> {
    passes: Vec<PassOp<P>>
}

impl<Pool: AsGeneratorPool> PassRunner<Pool> {
    /// The minimal pass runner permitted.
    pub fn core() -> Self {
        let mut runner = Self { passes: Vec::new() };
        runner.register::<WellFormed>(PassOptions::NONE);
        runner.register::<Canonicalize>(PassOptions::NONE);
        runner
    }

    /// The pass runner used by [`from_ast::ast_to_ir`]; notably, it does not
    /// preserve timing in control collasing.
    pub fn compile() -> Self {
        let mut runner = Self::core();
        runner.register_converge(10, |runner| {
            runner.register::<CopyProp>(PassOptions::NONE);
            runner.register::<DeadCode>(PassOptions::NONE);
            runner.register::<CollapseControl>(PassOptions::NONE);
        });
        runner.register::<CellAlloc>(PassOptions::NONE);
        runner
    }

    /// A pass runner for lowering a [`Component`] for an emission target,
    /// preserving timing.
    pub fn lower() -> Self {
        let mut runner = Self::core();
        runner.register::<RewriteAccesses>(PassOptions::PRESERVE_TIMING);
        runner.register_converge(10, |runner| {
            runner.register::<DeadCode>(PassOptions::PRESERVE_TIMING);
            runner.register::<CollapseControl>(PassOptions::PRESERVE_TIMING);
        });
        runner
    }

    pub fn register<P: Pass<Pool>>(&mut self, options: PassOptions) {
        self.passes.push(PassOp::Call(
            Box::new(|comp, pool, options| {
                let mut pass = P::from(options, comp, pool);
                // TODO: Allow opting in to reversed traversal
                pass.traverse_component(comp, pool, false)
            }),
            P::name(),
            options
        ));
    }

    pub fn register_converge<F: FnOnce(&mut Self)>(
        &mut self, iter_limit: usize, f: F
    ) {
        self.passes.push(PassOp::Coverge(iter_limit));
        f(self);
        self.passes.push(PassOp::EndConverge);
    }

    pub fn run(&mut self, comp: &mut Component, pool: &mut Pool) {
        let mut in_convergence = false;
        let mut convergence_iter_limit = 0;
        let mut convergence_region = Vec::new();

        for pass_op in &mut self.passes {
            match pass_op {
                PassOp::Call(pass, name, options) => {
                    log::info!(
                        "{}running pass '{}'",
                        if in_convergence { "  " } else { "" },
                        name
                    );
                    if in_convergence {
                        convergence_region.push((pass, *options));
                    } else {
                        pass(comp, pool, *options);
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
                                did_modify |= pass(comp, pool, *options);
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
