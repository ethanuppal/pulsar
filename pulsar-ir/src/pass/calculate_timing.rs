// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

use pulsar_utils::{id::Id, pool::Handle};

use crate::{
    component::ComponentViewMut,
    control::Control,
    from_ast::AsGeneratorPool,
    visitor::{Action, VisitorMut},
    Ir
};

use super::Pass;

pub struct CalculateTiming;

impl<P: AsGeneratorPool> VisitorMut<P> for CalculateTiming {
    fn start_enable(
        &mut self, id: Id, enable: &mut Ir, _comp_view: &mut ComponentViewMut,
        pool: &mut P
    ) -> Action {
        let control = Handle::<Control>::from_id(id, pool);
        pool.set_metadata(
            control,
            match enable {
                Ir::Add(_, _, _) => 0,
                Ir::Mul(_, _, _) => 4,
                Ir::Assign(_, _) => 0
            }
        );
        Action::None
    }
}

impl<P: AsGeneratorPool> Pass<P> for CalculateTiming {
    fn name(&self) -> &str {
        "calculate-timing"
    }
}
