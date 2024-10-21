//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    component::{Component, ComponentViewMut},
    control::{Control, Par, Seq},
    from_ast::AsGeneratorPool,
    port::Port,
    visitor::{Action, VisitorMut},
    Ir
};
use pulsar_utils::{id::Id, pool::Handle};
use std::collections::HashMap;

use super::{Pass, PassOptions};

pub struct CopyProp;

/// Replaces the port that `ir` assigns to with `new_kill`.
pub fn replace_kill(ir: &Ir, new_kill: Handle<Port>) -> Ir {
    match ir {
        Ir::Add(_, a, b) => Ir::Add(new_kill, *a, *b),
        Ir::Mul(_, a, b) => Ir::Mul(new_kill, *a, *b),
        Ir::Assign(_, b) => Ir::Assign(new_kill, *b)
    }
}

impl CopyProp {
    fn copy_prop(&self, seq: &mut [Handle<Control>]) {
        let mut assigns = HashMap::<Handle<Port>, Ir>::new();

        fn replace(
            assigns: &HashMap<Handle<Port>, Ir>, port: Handle<Port>
        ) -> Handle<Port> {
            *assigns
                .get(&port)
                .and_then(|ir| {
                    if let Ir::Assign(_, src) = ir {
                        Some(src)
                    } else {
                        None
                    }
                })
                .unwrap_or(&port)
        }

        for child in seq {
            if let Control::Enable(ir) = &mut **child {
                *ir = match *ir {
                    Ir::Add(kill, src1, src2) => Ir::Add(
                        kill,
                        replace(&assigns, src1),
                        replace(&assigns, src2)
                    ),
                    Ir::Mul(kill, src1, src2) => Ir::Mul(
                        kill,
                        replace(&assigns, src1),
                        replace(&assigns, src2)
                    ),
                    Ir::Assign(kill, src) => {
                        if let Some(latest_killer) = assigns.get(&src) {
                            replace_kill(latest_killer, kill)
                        } else {
                            Ir::Assign(kill, src)
                        }
                    }
                };
                assigns.insert(ir.kill(), ir.clone());
            }
        }
    }
}

impl<P: AsGeneratorPool> VisitorMut<P> for CopyProp {
    fn finish_seq(
        &mut self, _id: Id, seq: &mut Seq, _comp_view: &mut ComponentViewMut,
        _pool: &mut P
    ) -> Action {
        self.copy_prop(&mut seq.children);
        Action::None
    }

    fn finish_par(
        &mut self, _id: Id, par: &mut Par, _comp_view: &mut ComponentViewMut,
        _pool: &mut P
    ) -> Action {
        self.copy_prop(&mut par.children);
        Action::None
    }
}

impl<P: AsGeneratorPool> Pass<P> for CopyProp {
    fn name() -> &'static str {
        "copy-prop"
    }

    fn from(
        _options: PassOptions, _comp: &mut Component, _pool: &mut P
    ) -> Self {
        Self
    }
}
