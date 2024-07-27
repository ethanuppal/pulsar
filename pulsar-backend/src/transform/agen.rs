//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use std::ops::Deref;

use super::Transform;
use pulsar_ir::{
    cell::Cell,
    component::Component,
    control::Control,
    from_ast::AsGeneratorPool,
    label::{Label, Name, Visibility}
};

pub struct AddressGeneratorTransform;

impl<P: AsGeneratorPool> Transform<P> for AddressGeneratorTransform {
    fn apply(
        &mut self, comp: &Component, pool: &mut P
    ) -> anyhow::Result<Component> {
        let memories =
            comp.cells()
                .iter()
                .flat_map(|(var, cell)| match cell.deref() {
                    Cell::Memory(memory) => Some((var, memory)),
                    Cell::Register(_) => None
                });
        let label = Label::from(
            Name::from(format!("{}_agen", comp.label().name.unmangled())),
            Visibility::Public
        );

        let mut agen = Component::new(
            label,
            vec![],
            memories
                .map(|(var, memory)| {
                    (
                        *var,
                        pool.add(Cell::Register(
                            memory.flattened_address_width()
                        ))
                    )
                })
                .collect(),
            Control::Empty
        );

        println!("{}", agen);

        Ok(agen)
    }
}
