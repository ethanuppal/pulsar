//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::Transform;
use core::panic;
use pulsar_ir::{
    analysis::{timing::TimingAnalysis, Analysis},
    cell::{Cell, Direction},
    component::Component,
    control::{Control, ControlBuilder, For, Par, Seq},
    from_ast::AsGeneratorPool,
    label::{Label, Name, Visibility},
    memory::Memory,
    pass::PassRunner,
    port::Port,
    variable::Variable,
    visitor::Visitor,
    Ir
};
use pulsar_utils::{
    id::{Gen, Id},
    pool::Handle
};
use std::{
    collections::HashMap,
    iter,
    num::NonZeroUsize,
    ops::{Deref, DerefMut}
};

// ok tentative plan is
// and yes i know this is inefficient but
// i will go and extract all accesses so i get the program up to that access and
// duplicate it somehow
// or something it doesn't really matter
// then i schedule each access separately using my bottom-up-backward algorithm
// and ofc need to mark root for

struct AccessPort {
    port: Handle<Port>,
    memory: Memory,
    source: Handle<Control>,
    effective_parent: Option<Handle<Control>>,
    direction: Direction
}

#[derive(Default)]
struct AccessExtraction {
    effective_parent: Option<Handle<Control>>,
    dfs: Vec<Handle<Control>>,
    access_ports: Vec<AccessPort>
}

impl<P: AsGeneratorPool> Visitor<P> for AccessExtraction {
    fn start_enable(
        &mut self, id: Id, enable: &Ir, comp: &Component, pool: &P
    ) {
        for (port, direction) in iter::once((enable.kill(), Direction::WriteTo))
            .chain(
                enable
                    .gen_ref()
                    .iter()
                    .map(|port| (*port, Direction::ReadFrom))
            )
        {
            if let Port::Access(memory_var, ..) = &*port {
                let Cell::Memory(memory) = &*comp
                    .cells()
                    .get(memory_var)
                    .cloned()
                    .expect("CellAlloc pass not run")
                else {
                    panic!("tried to access non-memory in main component");
                };
                self.access_ports.push(AccessPort {
                    port,
                    memory: memory.clone(),
                    source: Handle::from_id(id, pool),
                    effective_parent: self.effective_parent,
                    direction
                });
            }
        }
    }

    fn start_control(&mut self, control: Handle<Control>, _pool: &P) {
        self.dfs.push(control);
        if self.dfs.len() >= 2 {
            if let Some(effective_parent) = self.effective_parent {
                if !matches!(&*effective_parent, Control::For(..)) {
                    self.effective_parent = Some(self.dfs[self.dfs.len() - 2]);
                }
            } else {
                self.effective_parent = Some(self.dfs[self.dfs.len() - 2]);
            }
        }
        // println!(
        //     "effective_parent = {}; control = {}",
        //     self.effective_parent
        //         .map(|x| x.to_string())
        //         .unwrap_or_default(),
        //     control
        // );
    }

    fn finish_control(&mut self, _control: Handle<Control>, _pool: &P) {
        if let Some(popped) = self.dfs.pop() {
            if let Some(effective_parent) = self.effective_parent {
                if popped.same_as(effective_parent)
                    && !matches!(&*popped, Control::For(..))
                {
                    self.effective_parent = None;
                }
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
    timing: TimingAnalysis,
    effective_parent: Option<Handle<Control>>,
    threads: Vec<AddressThread>
}

impl AddressGeneratorContext {
    pub fn new(memory_access_latency: usize, timing: TimingAnalysis) -> Self {
        Self {
            memory_access_latency,
            timing,
            effective_parent: None,
            threads: Vec::new()
        }
    }

    pub fn build_address_thread<P: AsGeneratorPool>(
        &mut self, comp: &Component, pool: &mut P, var_gen: &mut Gen,
        access_port: AccessPort
    ) {
        self.effective_parent = None;
        let mut builder = ControlBuilder::new(pool);
        self.build_control(
            &mut builder,
            var_gen,
            comp.cfg_pointer(),
            &access_port
        );
        self.time_travel(&mut builder, &access_port);
        let control = builder.take();
        self.threads.push(AddressThread {
            access_port,
            control: pool.add(control)
        });
    }

    fn build_for<P: AsGeneratorPool>(
        &mut self, builder: &mut ControlBuilder<P>, var_gen: &mut Gen,
        for_: &For, _id: Id, _control: Handle<Control>,
        access_port: &AccessPort
    ) -> Handle<Control> {
        let body = builder.with_pool(|pool| {
            let mut builder = ControlBuilder::new(pool);
            self.build_control(&mut builder, var_gen, for_.body(), access_port);
            let control = builder.take();
            pool.add(control)
        });
        builder.push(Control::For(For::new(
            for_.variant(),
            for_.lower_bound().clone(),
            for_.exclusive_upper_bound().clone(),
            for_.pipelined_ii,
            body
        )))
    }

    fn build_seq<P: AsGeneratorPool>(
        &mut self, builder: &mut ControlBuilder<P>, var_gen: &mut Gen,
        seq: &Seq, access_port: &AccessPort
    ) -> Handle<Control> {
        let new_seq = builder.with_pool(|pool| {
            let mut builder = ControlBuilder::new(pool);
            for child in seq.children() {
                self.build_control(&mut builder, var_gen, *child, access_port);
                builder.split();
            }
            builder.take()
        });
        builder.push(new_seq)
    }

    fn build_par<P: AsGeneratorPool>(
        &mut self, builder: &mut ControlBuilder<P>, var_gen: &mut Gen,
        par: &Par, access_port: &AccessPort
    ) -> Handle<Control> {
        let new_par = builder.with_pool(|pool| {
            let mut builder = ControlBuilder::new(pool);
            for child in par.children() {
                self.build_control(&mut builder, var_gen, *child, access_port);
            }
            builder.take()
        });
        builder.push(new_par)
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
        &mut self, builder: &mut ControlBuilder<P>, var_gen: &mut Gen, ir: &Ir,
        control: Handle<Control>, access_port: &AccessPort
    ) -> Handle<Control> {
        if access_port.source.same_as(control) {
            let Port::Access(memory_var, indices) = &*access_port.port else {
                panic!("invalid port extracted as memory access");
            };
            let inserted = builder.with_pool(|pool| {
                let mut builder = ControlBuilder::new(pool);
                // TODO: generate the multiplications and stuff to compute the
                // index
                let mut indices_rev = indices.iter().rev().cloned();
                let mut levels_rev =
                    access_port.memory.levels().iter().rev().cloned();
                let mut result_var = Variable::from(var_gen.next());

                let result_var_port =
                    builder.add_port(Port::Variable(result_var));
                builder.push(Ir::Assign(
                    result_var_port,
                    indices_rev.next().unwrap()
                ));

                // inductive step
                // ans = (ans + (index * length))
                for index in indices_rev {
                    let next_add = Variable::from(var_gen.next());
                    let next_add_port = builder
                        .with_pool(|pool| pool.add(Port::Variable(next_add)));
                    let length = builder
                        .new_const(levels_rev.next().unwrap().length as i64);
                    builder.push(Ir::Mul(next_add_port, index, length));
                    let new_result_var = Variable::from(var_gen.next());
                    builder.push_add(
                        next_add,
                        Port::Variable(result_var),
                        Port::Variable(next_add)
                    );
                    result_var = new_result_var;
                }

                builder.push_assign(
                    Port::Variable(*memory_var),
                    Port::Variable(result_var)
                );

                builder.take()
            });
            builder.push(inserted)
        } else {
            builder.push(ir.clone())
        }
    }

    fn build_control<P: AsGeneratorPool>(
        &mut self, builder: &mut ControlBuilder<P>, var_gen: &mut Gen,
        control: Handle<Control>, access_port: &AccessPort
    ) {
        let id = builder.with_pool(|pool| control.id_in(pool));
        let built_control = match &*control {
            Control::Empty =>  Handle::NULL,
            Control::Delay(_) =>
        panic!("There should be no `Control::Delay`s in the IR passed to address generation. Only lowering passes should generate delays."),
            Control::For(for_) => self.build_for(builder, var_gen, for_, id,control, access_port),
            Control::Seq(seq) => self.build_seq(builder, var_gen, seq, access_port),
            Control::Par(par) => self.build_par(builder, var_gen, par, access_port),
            Control::IfElse(_) => todo!(),
            Control::Enable(ir) => self.build_ir(builder, var_gen, ir, control, access_port)
        };
        if let Some(effective_parent) = access_port.effective_parent {
            if effective_parent.same_as(control) {
                self.effective_parent = Some(built_control);
            }
        }
    }

    fn time_travel<P: AsGeneratorPool>(
        &mut self, builder: &mut ControlBuilder<P>, access_port: &AccessPort
    ) {
        // match &mut *self.effective_parent.unwrap() {
        //     Control::For(_) => {}
        //     Control::Seq(seq) => ,
        //     Control::Par(_) => todo!(),
        //     Control::IfElse(_) => todo!(),
        //     Control::Enable(_) => todo!(),
        //     _ => unreachable!("delay or empty can't be parents")
        // }
        // actually don't think we need to do any of this since
        // there will be no other side effects lol
        // so just loops
        if let Some(Control::For(for_)) = self.effective_parent.as_deref_mut() {
            // TODO: assert that `inserted` has an II <= the original loop
            // latency
            let Control::For(original_for) =
                access_port.effective_parent.as_ref().unwrap().deref()
            else {
                unreachable!();
            };
            let original_effective_parent_body_id =
                builder.with_pool(|pool| original_for.body().id_in(pool));
            for_.pipelined_ii = NonZeroUsize::try_from(
                self.timing.get(original_effective_parent_body_id).latency()
            )
            .ok();
        } else {
            println!(
                "other effective parent: {}",
                access_port.effective_parent.unwrap()
            );
            println!("but current is: {}", self.effective_parent.unwrap());
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
            thread_shifts.iter().max().cloned().unwrap_or_default()
                + self.memory_access_latency;

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

        let timing = TimingAnalysis::for_comp(comp, pool);
        let mut agen_context = AddressGeneratorContext::new(10, timing);
        let access_ports = AccessExtraction::from(comp, pool).access_ports;
        for access_port in access_ports {
            agen_context.build_address_thread(comp, pool, var_gen, access_port);
        }
        let (cfg, global_shift) = agen_context.schedule(pool);

        let label = Label::from(
            Name::from(format!(
                "{}_agen_shift{}",
                comp.label().name.unmangled(),
                global_shift
            )),
            Visibility::Public
        );

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
