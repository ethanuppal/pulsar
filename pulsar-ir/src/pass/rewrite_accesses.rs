// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

use super::{Pass, PassOptions};
use crate::{
    component::{Component, ComponentViewMut},
    from_ast::AsGeneratorPool,
    port::Port,
    visitor::{Action, VisitorMut},
    Ir
};
use pulsar_utils::id::Id;
use std::ops::Deref;

pub struct RewriteAccesses;

impl<P: AsGeneratorPool> VisitorMut<P> for RewriteAccesses {
    fn start_enable(
        &mut self, _id: Id, enable: &mut Ir, _comp_view: &mut ComponentViewMut,
        _pool: &mut P
    ) -> Action {
        let mut did_modify = false;
        for port in enable.ports_mut() {
            if let Port::Access(var, _) = (*port).deref() {
                **port = Port::LoweredAccess(*var);
                did_modify = true;
            }
        }
        if did_modify {
            Action::ModifiedInternally
        } else {
            Action::None
        }
    }
}

impl<P: AsGeneratorPool> Pass<P> for RewriteAccesses {
    fn name() -> &'static str {
        "rewrite-accesses"
    }

    fn from(
        _options: PassOptions, __comp: &mut Component, _pool: &mut P
    ) -> Self {
        Self
    }
}
