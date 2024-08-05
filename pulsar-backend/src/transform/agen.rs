//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::Transform;
use calyx_ir::WRC;
use pulsar_ir::{
    cell::Cell,
    component::Component,
    control::{Control, ControlBuilder, For, Par, Seq},
    from_ast::AsGeneratorPool,
    label::{Label, Name, Visibility},
    memory::Memory,
    pass::{
        cell_alloc::CellAlloc, collapse_control::CollapseControl,
        copy_prop::CopyProp, dead_code::DeadCode, PassOptions, PassRunner
    },
    port::Port,
    variable::Variable,
    Ir
};
use pulsar_utils::id::Gen;
use std::{collections::HashMap, ops::Deref};

/// Synthesizes an address generator for a component.
pub struct AddressGeneratorTransform;

impl AddressGeneratorTransform {
    fn build_for<P: AsGeneratorPool>(
        &self, builder: &mut ControlBuilder<P>, for_: &For,
        memories: &HashMap<Variable, Memory>
    ) {
        let body = builder.with_pool(|pool| {
            let mut builder = ControlBuilder::new(pool);
            self.build_control(&mut builder, &for_.body(), memories);
            let control = builder.into();
            pool.add(control)
        });
        builder.push(Control::For(For::new(
            for_.variant(),
            for_.lower_bound().clone(),
            for_.exclusive_upper_bound().clone(),
            body
        )));
    }

    fn build_seq<P: AsGeneratorPool>(
        &self, builder: &mut ControlBuilder<P>, seq: &Seq,
        memories: &HashMap<Variable, Memory>
    ) {
        let new_seq = builder.with_pool(|pool| {
            let mut builder = ControlBuilder::new(pool);
            for child in seq.children() {
                self.build_control(&mut builder, child, memories);
                builder.split();
            }
            Control::from(builder)
        });
        builder.push(new_seq);
    }

    fn build_par<P: AsGeneratorPool>(
        &self, builder: &mut ControlBuilder<P>, par: &Par,
        memories: &HashMap<Variable, Memory>
    ) {
        let new_par = builder.with_pool(|pool| {
            let mut builder = ControlBuilder::new(pool);
            for child in par.children() {
                self.build_control(&mut builder, child, memories);
            }
            Control::from(builder)
        });
        builder.push(new_par);
    }

    /// The strategy behind this function is that the dead code pass will
    /// eliminate most of the unused instructions, so because I need to get this
    /// done now and I can fix it later, I'm going to:
    ///
    /// - Leave most of the IR in (since I'm going to need some of it to help
    ///   compute the addresses anyways).
    /// - Since I'm assuming no data-dependent addressing (TODO: make this an
    ///   invariant in `WellFormed` (for now since it will change)), I can
    ///   delete instructions that address or write to memory and replace them
    ///   with assignments to the address ports after computing their addresses.
    fn build_ir<P: AsGeneratorPool>(
        &self, builder: &mut ControlBuilder<P>, ir: &Ir,
        memories: &HashMap<Variable, Memory>
    ) {
        // since anything involving address access won't have side effects by
        // disallowing data-dependent addressing, we should be fine to ignore
        // all the one that do I think
        // no idea what I was writing here: let mut did_produce_access
        for port in ir.ports_ref() {
            if let Port::Access(array, indices) = &*port {
                assert!(indices.len() == 1, "disabled higher-d arrays for now");
                builder.push_assign(*array, indices[0].clone_out());
            }
        }
        builder.push(Control::Enable(ir.clone()));
    }

    fn build_control<P: AsGeneratorPool>(
        &self, builder: &mut ControlBuilder<P>, control: &Control,
        memories: &HashMap<Variable, Memory>
    ) {
        match control {
            Control::Empty => {}
            Control::Delay(_) => panic!(
                "There should be no delay control before lowering passes"
            ),
            Control::For(for_) => self.build_for(builder, for_, memories),
            Control::Seq(seq) => self.build_seq(builder, seq, memories),
            Control::Par(par) => self.build_par(builder, par, memories),
            Control::IfElse(_) => todo!(),
            Control::Enable(ir) => self.build_ir(builder, ir, memories)
        }
    }
}

impl<P: AsGeneratorPool> Transform<P> for AddressGeneratorTransform {
    fn apply(
        &mut self, comp: &Component, pool: &mut P, var_gen: &mut Gen
    ) -> anyhow::Result<Component> {
        let memories = comp
            .cells()
            .iter()
            .flat_map(|(var, cell)| match cell.deref() {
                Cell::Memory(memory) => Some((*var, memory.clone())),
                Cell::Register(_) => None
            })
            .collect::<HashMap<_, _>>();
        let label = Label::from(
            Name::from(format!("{}_agen", comp.label().name.unmangled())),
            Visibility::Public
        );

        let mut builder = ControlBuilder::new(pool);
        self.build_control(&mut builder, comp.cfg(), &memories);
        let cfg = Control::from(builder);

        let mut agen = Component::new(
            label,
            vec![],
            memories
                .iter()
                .map(|(var, memory)| {
                    (
                        *var,
                        pool.add(Cell::Register(
                            memory.flattened_address_width()
                        ))
                    )
                })
                .collect(),
            pool.add(cfg)
        );

        println!("??? {}", agen);

        // let mut runner = PassRunner::core();
        // runner.register_converge(10, |runner| {
        //     runner.register(CopyProp);
        //     runner.register(DeadCode::new(false));
        // // });
        // runner
        //     .register(CollapseControl::default(),
        // PassOptions::PRESERVE_TIMING); runner.register(CellAlloc,
        // PassOptions::PRESERVE_TIMING); runner.run(&mut agen, pool);

        Ok(agen)
    }
}
