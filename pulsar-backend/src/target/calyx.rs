//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::{OutputFile, Target};
use anyhow::anyhow;
use calyx_backend::{self as cback, Backend};
use calyx_builder::{
    finish_component, CalyxAssignmentContainer, CalyxBuilder, CalyxComponent,
    CalyxControl, CalyxControlType, CalyxPort, Sequential
};
use calyx_ir::{self as cir};
use calyx_opt as copt;
use calyx_utils as cutil;
use pulsar_ir::{
    cell::{Cell, Direction},
    component::Component,
    control::Control,
    from_ast::AsGeneratorPool,
    pass::cell_alloc::min_bits_to_represent,
    port::Port,
    variable::Variable,
    Ir
};
use pulsar_utils::{id::Id, pool::Handle};
use std::{
    collections::HashMap,
    env,
    fmt::{self, Debug, Display},
    io,
    ops::Deref,
    path::PathBuf
};

struct DisplayDebug<T: Debug>(T);

impl<T: Debug> Debug for DisplayDebug<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Debug> Display for DisplayDebug<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub struct CalyxTarget;

/// Usage: `calyx_attr![...]`, where `...` is a series of comma-separated tuples
/// of `Into<Attribute>`s and primitive integers, returns a `cir::Attributes`.
macro_rules! calyx_attr {
    ($(($attr:expr, $value:expr)),*) => {
        {
            let attrs = cir::Attributes::default();
            $(
                attrs.insert($attr, $value as u64);
            )*
            attrs
        }
    };
}

impl CalyxTarget {
    fn register_comp(
        &self, comp: &Component, comp_name: String, builder: &mut CalyxBuilder
    ) {
        let mut ports = Vec::new();
        for var in comp.inputs() {
            ports.push(cir::PortDef::new(
                var.to_string(),
                comp.cells().get(var).unwrap().port_width() as u64,
                cir::Direction::Input,
                calyx_attr![]
            ));
        }
        for var in comp.outputs() {
            ports.push(cir::PortDef::new(
                var.to_string(),
                comp.cells().get(var).unwrap().port_width() as u64,
                cir::Direction::Output,
                calyx_attr![]
            ));
        }
        builder.register_component(comp_name, ports);
    }

    fn emit_port<'a, 'b: 'a>(
        &self, port: &Port, direction: Direction,
        cells: &HashMap<Variable, Handle<Cell>>,
        calyx_comp: &'a mut CalyxComponent<'b, ()>
    ) -> CalyxPort {
        match port {
            Port::Constant(value) => {
                if direction == Direction::ReadFrom {
                    calyx_comp.constant(*value, 64).get("out")
                } else {
                    panic!("constants have no input ports")
                }
            }
            Port::Variable(var) => {
                let calyx_cell = if cells.contains_key(var) {
                    calyx_comp.find(var.to_string())
                } else {
                    calyx_comp.new_unnamed_prim("std_wire", vec![64])
                };
                calyx_cell.get(if direction == Direction::WriteTo {
                    "in"
                } else {
                    "out"
                })
            }
            Port::LoweredAccess(array) => {
                calyx_comp.signature().get(array.to_string())
            }
            Port::PartialAccess(..) => {
                panic!("run the Canonicalize pass (got {:?})", port)
            }
            Port::Access(..) => {
                panic!("run the RewriteAccesses pass")
            }
        }
    }

    fn emit_ir<'a, 'b: 'a, T: CalyxControlType, P: AsGeneratorPool>(
        &self, id: Id, ir: &Ir, cells: &HashMap<Variable, Handle<Cell>>,
        calyx_control: &mut CalyxControl<T>,
        calyx_comp: &'a mut CalyxComponent<'b, ()>, pool: &P
    ) {
        let latency = {
            // technically both `control` and `ir` own the same thing
            let control = Handle::<Control>::from_id(id, pool);
            *pool.get_metadata(control)
        };
        let latency = if latency == 0 { 1 } else { latency };

        match ir {
            Ir::Add(result, lhs, rhs) => {
                let result = self.emit_port(
                    result,
                    Direction::WriteTo,
                    cells,
                    calyx_comp
                );
                let lhs =
                    self.emit_port(lhs, Direction::ReadFrom, cells, calyx_comp);
                let rhs =
                    self.emit_port(rhs, Direction::ReadFrom, cells, calyx_comp);
                let adder = calyx_comp.new_unnamed_prim("std_add", vec![64]);
                let mut group = calyx_comp.add_static_group("ir_add", latency);
                group.assign(adder.get("left"), lhs);
                group.assign(adder.get("right"), rhs);
                group.assign(result, adder.get("out"));
                calyx_control.insert_static(&group);
            }
            Ir::Mul(result, lhs, rhs) => {
                let result = self.emit_port(
                    result,
                    Direction::WriteTo,
                    cells,
                    calyx_comp
                );
                let lhs =
                    self.emit_port(lhs, Direction::ReadFrom, cells, calyx_comp);
                let rhs =
                    self.emit_port(rhs, Direction::ReadFrom, cells, calyx_comp);
                let multiplier =
                    calyx_comp.new_unnamed_prim("std_mult_pipe", vec![64]);
                let mut group =
                    calyx_comp.add_static_group("ir_multiply", latency);
                group.assign(multiplier.get("left"), lhs);
                group.assign(multiplier.get("right"), rhs);
                group.assign(result, multiplier.get("out"));
                calyx_control.insert_static(&group);
            }
            Ir::Assign(lhs, rhs) => {
                let lhs =
                    self.emit_port(lhs, Direction::WriteTo, cells, calyx_comp);
                let rhs =
                    self.emit_port(rhs, Direction::ReadFrom, cells, calyx_comp);
                let mut group =
                    calyx_comp.add_static_group("ir_assign", latency);
                group.assign(lhs, rhs);
                calyx_control.insert_static(&group);
            }
        }
    }

    fn emit_control<'a, 'b: 'a, T: CalyxControlType, P: AsGeneratorPool>(
        &self, id: Id, control: &Control,
        cells: &HashMap<Variable, Handle<Cell>>,
        calyx_control: &mut CalyxControl<T>,
        calyx_comp: &'a mut CalyxComponent<'b, ()>, pool: &P
    ) {
        match control {
            Control::Empty => {}
            Control::Delay(delay) => {
                let delay = calyx_comp.add_static_group("delay", *delay);
                calyx_control.insert_static(&delay);
            }
            Control::For(for_) => calyx_control.seq(|calyx_control| {
                let index = calyx_comp.find(for_.variant().to_string());
                let lower = self.emit_port(
                    for_.lower_bound(),
                    Direction::ReadFrom,
                    cells,
                    calyx_comp
                );
                let exclusive_upper = self.emit_port(
                    for_.exclusive_upper_bound(),
                    Direction::ReadFrom,
                    cells,
                    calyx_comp
                );
                let index_bits =
                    cells.get(&for_.variant()).unwrap().port_width();

                let mut init_index = calyx_comp
                    .add_static_group("init_index", for_.init_latency());
                init_index.assign(index.get("in"), lower);
                let signal_out = calyx_comp.signal_out();
                init_index.assign(index.get("write_en"), signal_out.get("out"));
                calyx_control.insert_static(&init_index);

                let lt = calyx_comp
                    .new_unnamed_prim("std_lt", vec![index_bits as u64]);
                let mut check_cond = calyx_comp.add_comb_group("check_cond");
                check_cond.assign(lt.get("left"), index.get("out"));
                check_cond.assign(lt.get("right"), exclusive_upper);
                calyx_control.while_(
                    lt.get("out"),
                    Some(check_cond),
                    |calyx_control| {
                        self.emit_control(
                            for_.body().id_in(pool),
                            &for_.body(),
                            cells,
                            calyx_control,
                            calyx_comp,
                            pool
                        );
                    }
                );
            }),
            Control::Seq(seq) => {
                calyx_control.seq(|calyx_control| {
                    for child in seq.children() {
                        self.emit_control(
                            child.id_in(pool),
                            child,
                            cells,
                            calyx_control,
                            calyx_comp,
                            pool
                        );
                    }
                });
            }
            Control::Par(par) => {
                calyx_control.par(|calyx_control| {
                    for child in par.children() {
                        self.emit_control(
                            child.id_in(pool),
                            child,
                            cells,
                            calyx_control,
                            calyx_comp,
                            pool
                        );
                    }
                });
            }
            Control::IfElse(if_else) => todo!(),
            Control::Enable(ir) => {
                self.emit_ir(id, ir, cells, calyx_control, calyx_comp, pool);
            }
        }
    }

    fn emit_comp<'a, 'b: 'a, P: AsGeneratorPool>(
        &self, comp: &Component, calyx_comp: &'a mut CalyxComponent<'b, ()>,
        pool: &P
    ) {
        for (var, cell) in comp.internal_cells() {
            match cell.deref() {
                Cell::Memory(_) => {}
                Cell::Register(width) => {
                    calyx_comp.new_reg(var.to_string(), *width);
                }
            }
        }

        let mut calyx_control = CalyxControl::<Sequential>::default();
        self.emit_control(
            comp.cfg_id(pool),
            comp.cfg(),
            comp.cells(),
            &mut calyx_control,
            calyx_comp,
            pool
        );
        *calyx_comp.control() = calyx_control;
    }
}

impl<P: AsGeneratorPool> Target<P> for CalyxTarget {
    fn emit(
        &mut self, comp: &Component, pool: &P, output: super::OutputFile
    ) -> anyhow::Result<()> {
        let mut prelude_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        prelude_file_path.push("resources");
        prelude_file_path.push("prelude.futil");

        let mut lib_path = PathBuf::from(env::var("HOME")?);
        lib_path.push(".calyx");

        let comp_name = comp.label().name.unmangled().to_string(); //.mangled()

        let mut builder = CalyxBuilder::new(
            Some(prelude_file_path),
            lib_path,
            Some(comp_name.clone()),
            "_".into()
        )
        .expect("Invalid library path");

        self.register_comp(comp, comp_name.clone(), &mut builder);
        let mut calyx_comp = builder.start_component::<()>(comp_name);
        self.emit_comp(comp, &mut calyx_comp, pool);
        finish_component!(builder, calyx_comp);

        let mut calyx_ctx = builder.finalize();

        cir::Printer::write_context(&calyx_ctx, true, &mut io::stderr())
            .unwrap();

        let pm = copt::pass_manager::PassManager::default_passes()
            .map_err(|e| anyhow!(DisplayDebug(e)))?;
        calyx_ctx.bc = cir::BackendConf {
            synthesis_mode: false,
            enable_verification: false,
            flat_assign: true,
            emit_primitive_extmodules: false
        };
        pm.execute_plan(
            &mut calyx_ctx,
            &[
                "well-formed".to_string(),
                "compile".to_owned(),
                "lower".to_string()
            ],
            &[],
            false
        )
        .map_err(|e| anyhow!(DisplayDebug(e)))?;

        // let backend = cback::VerilogBackend;
        // backend
        //     .run(calyx_ctx, output.into())
        //     .map_err(|e| anyhow!(DisplayDebug(e)))?;
        Ok(())
    }
}

impl From<OutputFile> for cutil::OutputFile {
    fn from(output: OutputFile) -> Self {
        match output {
            OutputFile::Stdout => cutil::OutputFile::Stdout,
            OutputFile::Stderr => cutil::OutputFile::Stderr,
            OutputFile::File(path) => cutil::OutputFile::File(path)
        }
    }
}
