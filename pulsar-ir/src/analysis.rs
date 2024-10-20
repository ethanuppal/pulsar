//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    component::Component, from_ast::AsGeneratorPool, visitor::Visitor
};

pub mod side_effect;
pub mod timing;

pub trait Analysis {
    fn for_comp<P: AsGeneratorPool>(
        comp: &mut Component, pool: &mut P
    ) -> Self
    where
        Self: Default + Visitor<P> {
        let mut new_self = Self::default();
        new_self.traverse_component(comp, pool, false);
        new_self
    }
}
