//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    component::ComponentViewMut,
    control::{Control, Par, Seq},
    from_ast::AsGeneratorPool,
    port::Port,
    visitor::{Action, Visitor},
    Ir
};
use match_deref::match_deref;
use pulsar_utils::pool::Handle;
use std::ops::Deref;

/// This pass is applied after `WellFormed` and before all other passes.
///
///  Incoming invariants:
/// - The IR has been freshly created via recursive
///   `ComponentGenerator::gen_stmt`s.
/// - All array accesses are gated behind [`Ir::Assign`]s.
///
/// Effects:
/// - Folds chains of [`Operand::PartialAccess`]s into a single
///   [`Operand::Access`] ([`Canonicalize::collapse_partial_accesses`]).
pub struct Canonicalize;

impl Canonicalize {
    /// Either the original control pair returned unchanged on the left or a
    /// replacement on the right.
    fn collapse_partial_access_pair<P: AsGeneratorPool>(
        &self, first: &Control, second: &Control, pool: &mut P
    ) -> Option<Control> {
        match_deref! {
            match &(first, second) {
                (
                    Control::Enable(Ir::Assign(
                        Deref @_temp,
                        Deref @Port::PartialAccess(Deref @array, index)
                    )),
                    Control::Enable(Ir::Assign(
                        result,
                        Deref @Port::PartialAccess(Deref @_temp_again, index2)
                    ))
                ) if _temp_again == _temp => {
                    let Port::Variable(array_var) = *array else {
                        panic!("not well-formed: first partial assign indexes non-variable")
                    };
                    Some(Control::Enable(Ir::Assign(
                        *result,
                        pool.add(Port::Access(
                            array_var,
                            vec![*index, *index2]
                        ))
                    )))
                }
                (
                    Control::Enable(Ir::Assign(
                        Deref @_temp,
                        Deref @Port::Access(array, indices)
                    )),
                    Control::Enable(Ir::Assign(
                        result,
                        Deref @Port::PartialAccess(Deref @_temp_again, index2)
                    ))
                ) if _temp_again == _temp => {
                    let mut indices = indices.to_owned();
                    indices.push(*index2);
                    Some(Control::Enable(Ir::Assign(
                        *result,
                        pool.add(Port::Access(*array, indices))
                    )))
                }
                _ => None
            }
        }
    }

    fn collapse_partial_accesses<P: AsGeneratorPool>(
        &self, children: &mut Vec<Handle<Control>>, pool: &mut P
    ) {
        // first, take pairs of PartialAccess/PartialAccess and
        // Access/PartialAccess and fold them up into Access
        let mut i = 0;
        let mut advance_i;
        while i < children.len() {
            advance_i = 1;
            if i < children.len() - 1 {
                if let Some(replace) = self.collapse_partial_access_pair(
                    &children[i],
                    &children[i + 1],
                    pool
                ) {
                    *children[i] = replace;
                    children.remove(i + 1);
                    advance_i = 0;
                }
            }
            i += advance_i;
        }

        // then, since only individual PartialAccess will remain PartialAccess,
        // we can just turn them into Access
        for i in 0..(children.len() as isize) - 1 {
            if let Control::Enable(Ir::Assign(result, access)) =
                children[i as usize].deref()
            {
                if let Port::PartialAccess(array, index) = access.deref() {
                    let Port::Variable(array_var) = **array else {
                        panic!("not well-formed: first partial assign indexes non-variable")
                    };
                    *children[i as usize] = Control::Enable(Ir::Assign(
                        *result,
                        pool.add(Port::Access(array_var, vec![*index]))
                    ));
                }
            }
        }
    }
}

impl<P: AsGeneratorPool> Visitor<P> for Canonicalize {
    fn start_seq(
        &mut self, seq: &mut Seq, _comp_view: &mut ComponentViewMut, pool: &mut P
    ) -> Action {
        self.collapse_partial_accesses(&mut seq.children, pool);
        Action::None
    }

    fn start_par(
        &mut self, par: &mut Par, _comp_view: &mut ComponentViewMut, pool: &mut P
    ) -> Action {
        self.collapse_partial_accesses(&mut par.children, pool);
        Action::None
    }
}
