//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    control::{Control, Par, Seq},
    from_ast::AsGeneratorPool,
    operand::Operand,
    variable::Variable,
    visitor::{Action, Visitor},
    Ir
};
use pulsar_utils::pool::Handle;
use std::{
    cmp,
    collections::HashMap,
    ops::{Deref, DerefMut}
};

pub struct CopyProp;

impl CopyProp {
    fn copy_prop(&self, seq: &mut [Handle<Control>]) {
        let mut assigns = HashMap::<Variable, Operand>::new();

        fn replace(
            assigns: &HashMap<Variable, Operand>, operand: &Operand
        ) -> Operand {
            if let Operand::Variable(var) = operand {
                assigns.get(var).unwrap_or(operand)
            } else {
                operand
            }
            .clone()
        }

        for child in seq {
            if let Control::Enable(ir) = &mut **child {
                *ir = match ir {
                    Ir::Add(kill, src1, src2) => Ir::Add(
                        *kill,
                        replace(&assigns, src1),
                        replace(&assigns, src2)
                    ),
                    Ir::Mul(kill, src1, src2) => Ir::Mul(
                        *kill,
                        replace(&assigns, src1),
                        replace(&assigns, src2)
                    ),
                    Ir::Assign(kill, src) => {
                        let src = replace(&assigns, src);
                        if let Operand::Variable(kill) = kill {
                            assigns.insert(*kill, src.clone());
                        }
                        Ir::Assign(kill.clone(), src)
                    }
                }
            }
        }
    }
}

impl<P: AsGeneratorPool> Visitor<P> for CopyProp {
    fn finish_seq(&mut self, seq: &mut Seq, pool: &mut P) -> Action {
        self.copy_prop(&mut seq.children);
        Action::None
    }

    fn finish_par(&mut self, par: &mut Par, pool: &mut P) -> Action {
        self.copy_prop(&mut par.children);
        Action::None
    }
}
