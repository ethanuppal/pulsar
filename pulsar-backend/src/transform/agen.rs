//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::Transform;
use core::panic;
use pulsar_ir::{
    cell::{Cell, Direction},
    component::{Component, ComponentViewMut},
    control::{Control, ControlBuilder, For, Par, Seq},
    from_ast::AsGeneratorPool,
    label::{Label, Name, Visibility},
    memory::Memory,
    pass::PassRunner,
    port::Port,
    variable::Variable,
    visitor::{Action, Visitor, VisitorMut},
    Ir
};
use pulsar_utils::{
    id::{Gen, Id},
    pool::Handle
};
use std::{collections::HashMap, iter, ops::Deref, os::unix::thread};

// ok tentative plan is
// and yes i know this is inefficient but
// i will go and extract all accesses so i get the program up to that access and
// duplicate it somehow
// or something it doesn't really matter
// then i schedule each access separately using my bottom-up-backward algorithm
// and ofc need to mark root for

struct AccessPort {
    port: Handle<Port>,
    source: Handle<Control>,
    direction: Direction
}

#[derive(Default)]
struct AccessExtraction {
    access_ports: Vec<AccessPort>
}

impl<P: AsGeneratorPool> Visitor<P> for AccessExtraction {
    fn start_enable(
        &mut self, id: Id, enable: &Ir, _comp: &Component, pool: &P
    ) {
        for (port, direction) in iter::once((enable.kill(), Direction::WriteTo))
            .chain(
                enable
                    .gen_ref()
                    .iter()
                    .map(|port| (*port, Direction::ReadFrom))
            )
        {
            if matches!(&*port, Port::Access(..)) {
                self.access_ports.push(AccessPort {
                    port,
                    source: Handle::from_id(id, pool),
                    direction
                });
            }
        }
    }
}

impl AccessExtraction {
    pub fn from<P: AsGeneratorPool>(comp: &Component, pool: &P) -> Self {
        let mut new_self = Self::default();
        new_self.traverse_component(comp, pool, false);
        new_self
    }
}

pub struct AddressThread {
    access_port: AccessPort,
    control: Handle<Control>
}

// ETHAN: I'm realizing this current impl won't work since we need to identify
// the e.g. effective for-loop parent before doing anything else right? before
// copying over? wait maybe not. we can just replace the access with our
// calculation, set the loop forced II, and it works. yeah

struct AddressGeneratorContext {
    // Assumed to be constant
    memory_access_latency: usize,
    /// The node at which time-travel must occur.
    effective_parent: Option<Handle<Control>>,
    dfs: Vec<Handle<Control>>,
    threads: Vec<AddressThread>
}

impl AddressGeneratorContext {
    pub fn new(memory_access_latency: usize) -> Self {
        Self {
            memory_access_latency,
            effective_parent: None,
            dfs: Vec::new(),
            threads: Vec::new()
        }
    }

    pub fn build_address_thread<P: AsGeneratorPool>(
        &mut self, comp: &Component, pool: &mut P, access_port: AccessPort
    ) {
    }

    fn build_for<P: AsGeneratorPool>(
        &mut self, builder: &mut ControlBuilder<P>, for_: &For
    ) {
        let body = builder.with_pool(|pool| {
            let mut builder = ControlBuilder::new(pool);
            self.build_control(&mut builder, for_.body());
            let control = builder.take();
            pool.add(control)
        });
        builder.push(Control::For(For::new(
            for_.variant(),
            for_.lower_bound().clone(),
            for_.exclusive_upper_bound().clone(),
            todo!(),
            body
        )));
    }

    fn build_seq<P: AsGeneratorPool>(
        &mut self, builder: &mut ControlBuilder<P>, seq: &Seq
    ) {
        let new_seq = builder.with_pool(|pool| {
            let mut builder = ControlBuilder::new(pool);
            for child in seq.children() {
                self.build_control(&mut builder, *child);
                builder.split();
            }
            builder.take()
        });
        builder.push(new_seq);
    }

    fn build_par<P: AsGeneratorPool>(
        &mut self, builder: &mut ControlBuilder<P>, par: &Par
    ) {
        let new_par = builder.with_pool(|pool| {
            let mut builder = ControlBuilder::new(pool);
            for child in par.children() {
                self.build_control(&mut builder, *child);
            }
            builder.take()
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
        &mut self, builder: &mut ControlBuilder<P>, ir: &Ir
    ) {
        // since anything involving address access won't have side effects by
        // disallowing data-dependent addressing, we should be fine to ignore
        // all the one that do I think
        let mut did_produce_access = false;
        for port in ir.ports_ref() {
            if let Port::Access(array, indices) = &*port {
                did_produce_access = true;
                assert!(indices.len() == 1, "disabled higher-d arrays for now");
                builder.push_assign(*array, indices[0].clone_out());
            }
        }
        if !did_produce_access {
            builder.push(Control::Enable(ir.clone()));
        }
    }

    fn build_control<P: AsGeneratorPool>(
        &mut self, builder: &mut ControlBuilder<P>, control: Handle<Control>
    ) {
        self.dfs.push(control);
        if self.dfs.len() >= 2 {
            if let Some(effective_parent) = self.effective_parent {
                if !matches!(&*effective_parent, Control::For(..)) {
                    self.effective_parent = Some(self.dfs[self.dfs.len() - 2]);
                }
            } else {
                self.effective_parent =
                    Some(self.dfs[self.dfs.len() - 2 /* always 0 */]);
            }
        }
        match &*control {
            Control::Empty => {}
            Control::Delay(_) =>
        panic!("There should be no `Control::Delay`s in the IR passed to address generation. Only lowering passes should generate delays."),
            Control::For(for_) => self.build_for(builder, for_),
            Control::Seq(seq) => self.build_seq( builder, seq),
            Control::Par(par) => self.build_par( builder, par),
            Control::IfElse(_) => todo!(),
            Control::Enable(ir) => self.build_ir( builder, ir)
        }
        self.dfs.pop().expect("somehow recursion got messed up");
        if self.dfs.len() < 2 {
            self.effective_parent = None;
        }
    }

    /// Schedules the built address threads and the latency to initiate the
    /// schedule in advance by.
    pub fn schedule<P: AsGeneratorPool>(
        self, pool: &mut P
    ) -> (Handle<Control>, usize) {
        let thread_shifts = self
            .threads
            .iter()
            .map(|thread| match thread.access_port.direction {
                Direction::WriteTo => {
                    thread.access_port.port.expansion_latency()
                }
                Direction::ReadFrom => {
                    self.memory_access_latency
                        + thread.access_port.port.expansion_latency()
                }
            })
            .collect::<Vec<_>>();

        let global_shift =
            thread_shifts.iter().max().cloned().unwrap_or_default();

        let mut scheduled_threads = Vec::new();
        for (i, thread) in self.threads.into_iter().enumerate() {
            let delay =
                pool.add(Control::Delay(global_shift - thread_shifts[i]));
            let scheduled_thread =
                pool.add(Control::seq([delay, thread.control]));
            scheduled_threads.push(scheduled_thread);
        }
        (pool.add(Control::par(scheduled_threads)), global_shift)
    }
}

/// Synthesizes an address generator for a component.
pub struct AddressGeneratorTransform;

impl<P: AsGeneratorPool> Transform<P> for AddressGeneratorTransform {
    fn apply(
        &mut self, comp: &Component, pool: &mut P, _var_gen: &mut Gen
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

        let mut agen_context = AddressGeneratorContext::new(10);
        let access_ports = AccessExtraction::from(comp, pool).access_ports;
        for access_port in access_ports {
            agen_context.build_address_thread(comp, pool, access_port);
        }
        let (cfg, global_shift) = agen_context.schedule(pool);

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
            cfg
        );

        PassRunner::lower().run(&mut agen, pool);

        Ok(agen)
    }
}
