//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    control::{Control, Par, Seq},
    from_ast::AsGeneratorPool,
    operand::Operand,
    visitor::{Action, Visitor},
    Ir
};
use either::Either::{self, Left, Right};
use pulsar_utils::pool::Handle;
use std::{cmp, mem, ops::Deref};

/// This pass is applied before all other passes.
///
///  Incoming invariants:
/// - The IR has been freshly created via recursive
///   `ComponentGenerator::gen_stmt`s.
/// - All array accesses are gated behind [`Ir::Assign`]s.
///
/// Effects:
/// - Folds chains of [`Operand::PartialAccess`]s into a single
///   [`Operand::Access`] ([`Canonicalize::collapse_partial_accesses`]).
// - Re-orders IR statements in parallel control in the order that their
//   left-hand-side variables were generated ([`Canonicalize::finish_par`]).
pub struct Canonicalize;

impl Canonicalize {
    /// Either the original control pair returned unchanged on the left or a
    /// replacement on the right.
    fn collapse_partial_access_pair(
        &self, first: Control, second: Control
    ) -> Either<(Control, Control), Control> {
        match (first, second) {
            (
                Control::Enable(Ir::Assign(
                    temp,
                    Operand::PartialAccess(array, index)
                )),
                Control::Enable(Ir::Assign(
                    result,
                    Operand::PartialAccess(temp_again, index2)
                ))
            ) if *temp_again == temp => {
                let Operand::Variable(array_var) = *array else {
                    panic!("not well-formed: first partial assign indexes non-variable")
                };
                Right(Control::Enable(Ir::Assign(
                    result,
                    Operand::Access(
                        array_var,
                        vec![*index.clone(), *index2.clone()]
                    )
                )))
            }
            (
                Control::Enable(Ir::Assign(
                    temp,
                    Operand::Access(array, mut indices)
                )),
                Control::Enable(Ir::Assign(
                    result,
                    Operand::PartialAccess(temp_again, index2)
                ))
            ) if *temp_again == temp => {
                indices.push(*index2);
                Right(Control::Enable(Ir::Assign(
                    result,
                    Operand::Access(array.clone(), indices)
                )))
            }
            other => Left(other)
        }
    }

    fn collapse_partial_accesses(&self, children: &mut Vec<Handle<Control>>) {
        // first, take pairs of PartialAccess/PartialAccess and
        // Access/PartialAccess and fold them up into Access
        let mut i = 0;
        let mut advance_i;
        while i < children.len() {
            advance_i = 1;
            if i < children.len() - 1 {
                match self.collapse_partial_access_pair(
                    mem::take(&mut children[i]),
                    mem::take(&mut children[i + 1])
                ) {
                    Left((old_first, old_second)) => {
                        *children[i] = old_first;
                        *children[i + 1] = old_second;
                    }
                    Right(replace) => {
                        *children[i] = replace;
                        children.remove(i + 1);
                        advance_i = 0;
                    }
                }
            }
            i += advance_i;
        }

        // then, since only individual PartialAccess will remain PartialAccess,
        // we can just turn them into Access
        for i in 0..children.len() - 1 {
            if let Control::Enable(Ir::Assign(
                result,
                Operand::PartialAccess(array, index)
            )) = children[i].deref()
            {
                let Operand::Variable(array_var) = **array else {
                    panic!("not well-formed: first partial assign indexes non-variable")
                };
                *children[i] = Control::Enable(Ir::assign(
                    result.clone(),
                    Operand::Access(array_var, vec![*index.clone()])
                ));
            }
        }
    }
}

impl<P: AsGeneratorPool> Visitor<P> for Canonicalize {
    fn start_seq(&mut self, seq: &mut Seq, _pool: &mut P) -> Action {
        self.collapse_partial_accesses(&mut seq.children);
        Action::None
    }

    fn start_par(&mut self, par: &mut Par, _pool: &mut P) -> Action {
        self.collapse_partial_accesses(&mut par.children);

        // par.children.sort_by(|a, b| match (a.deref(), b.deref()) {
        //     (Control::Enable(ir_a), Control::Enable(ir_b)) => {
        //         match (ir_a.kill(), ir_b.kill()) {
        //             (Operand::Variable(var_a), Operand::Variable(var_b)) => {
        //                 var_a.cmp(&var_b)
        //             }
        //             _ => cmp::Ordering::Less
        //         }
        //     }
        //     _ => cmp::Ordering::Less
        // });

        Action::None
    }
}
