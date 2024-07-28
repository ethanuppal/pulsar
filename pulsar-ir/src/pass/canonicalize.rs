//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    component::ComponentViewMut,
    control::{Control, Par, Seq},
    from_ast::AsGeneratorPool,
    port::Port,
    visitor::{Action, VisitorMut},
    Ir
};
use match_deref::match_deref;
use pulsar_utils::{id::Id, pool::Handle};
use std::ops::{Deref, DerefMut};

use super::{copy_prop::replace_kill, Pass};

fn fold_partial_access<P: AsGeneratorPool>(
    mut port: Handle<Port>, pool: &mut P
) -> Handle<Port> {
    println!("folding: {}", port);
    if let Port::PartialAccess(..) = port.deref() {
        let mut indices = Vec::new();
        while let Port::PartialAccess(inner, index) = port.deref() {
            indices.push(*index);
            port = *inner;
        }
        let Port::Variable(array) = port.deref() else {
            panic!("partial access chain resolved to non-variable array")
        };
        pool.add(Port::Access(*array, indices))
    } else {
        port
    }
}

/// This pass is applied after `WellFormed` and before all other passes.
///
///  Incoming invariants:
/// - The IR has been freshly created via recursive
///   `ComponentGenerator::gen_stmt`s.
///     - Partial access chains exist on lhs ports as fully resolved (because
///       ports are lvalues).
/// - All array accesses are gated behind [`Ir::Assign`]s.
///
/// Effects:
/// - Folds chains of [`Port::PartialAccess`]s into a single [`Port::Access`]
///   ([`Canonicalize::collapse_partial_accesses`]).
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

        // then, since only individual/nested PartialAccess will remain
        // PartialAccess, we can just turn them into Access
        for child in children {
            if let Control::Enable(ir) = child.deref_mut() {
                for gen in ir.gen_mut() {
                    *gen = fold_partial_access(*gen, pool);
                }
                let new_kill = fold_partial_access(ir.kill(), pool);
                *ir = replace_kill(ir, new_kill);
            }
        }
    }
}

impl<P: AsGeneratorPool> VisitorMut<P> for Canonicalize {
    fn start_seq(
        &mut self, _id: Id, seq: &mut Seq, _comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        self.collapse_partial_accesses(&mut seq.children, pool);
        Action::ModifiedInternally
    }

    fn start_par(
        &mut self, _id: Id, par: &mut Par, _comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        self.collapse_partial_accesses(&mut par.children, pool);
        Action::ModifiedInternally
    }
}

impl<P: AsGeneratorPool> Pass<P> for Canonicalize {
    fn name(&self) -> &str {
        "canonicalize"
    }
}
