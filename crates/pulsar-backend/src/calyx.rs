// Copyright (C) 2024 Ethan Uppal. All rights reserved.

use super::PulsarBackend;
use crate::Output;
use builder::{CalyxBuilder, CalyxCell, CalyxComponent};
use calyx_backend::Backend;
use calyx_ir::{build_assignments, RRC};
use pulsar_frontend::ty::Type;
use pulsar_ir::{
    basic_block::BasicBlockCell,
    control_flow_graph::ControlFlowGraph,
    generator::GeneratedTopLevel,
    label::{Label, LabelName},
    operand::Operand,
    variable::Variable,
    Ir
};
use std::{io::stderr, path::PathBuf};
use crate::build_assignments_2;

pub mod builder;

// This file contains many examples of BAD software engineering.
//
// One boilerplate I'm noticing is in construction of cells, groups, and control
// -- see if you can find a nicer, more beautiful way to write it
//
// All components are treated very much like functions. They have input ports
// all of width 64 and one output port of width 64. However hardware is a lot
// more flexible than this. See if you can figure out how to better use it.
//
// I realized that a contributing factor to this is that my IR has everything
// has Int64. I should change that

struct FunctionContext {
    ret_cell: Option<CalyxCell>,
    param_env: usize
}

impl Default for FunctionContext {
    fn default() -> Self {
        Self {
            ret_cell: None,
            param_env: 0
        }
    }
}

pub struct CalyxBackend<'a> {
    builder: CalyxBuilder<'a, FunctionContext>
}

impl<'a> CalyxBackend<'a> {
    /// Returns the register associated with `var` in the current component,
    /// building it if necessary.
    fn cell_for_var(
        &mut self, component: &'a mut CalyxComponent<'a, FunctionContext>,
        var: Variable
    ) -> CalyxCell {
        component.named_reg(var.to_string(), 64)
    }

    // TODO: support larger nested arrays
    // but arbitrary pointer access needs to be restricted in static analyzer
    // when targeting hardware
    fn make_cell_for_pointer(
        &mut self, component: &'a mut CalyxComponent<'a, FunctionContext>,
        var: Variable, cell_size: usize, length: usize
    ) -> CalyxCell {
        component.named_mem(var.to_string(), cell_size, length, 64)
    }

    /// Builds a constant if the operand is a constant. See
    /// [`CalyxBackend::cell_for_var`] for when the operand is a variable.
    fn cell_for_operand(
        &mut self, component: &'a mut CalyxComponent<'a, FunctionContext>,
        operand: &Operand
    ) -> CalyxCell {
        match &operand {
            Operand::Constant(value) => component.constant(*value, 64),
            Operand::Variable(var) => self.cell_for_var(component, *var)
        }
    }

    fn cache_func_sig(
        &mut self, label: &Label, args: &Vec<Type>, ret: &Box<Type>
    ) {
        let mut comp_ports = vec![];
        for (i, arg) in args.iter().enumerate() {
            let width = arg.size();
            let name = format!("arg{}", i);
            comp_ports.push(calyx_ir::PortDef::new(
                name,
                (width * 8) as u64,
                calyx_ir::Direction::Input,
                calyx_ir::Attributes::default()
            ));
        }
        if **ret != Type::Unit {
            comp_ports.push(calyx_ir::PortDef::new(
                "ret",
                (ret.size() * 8) as u64,
                calyx_ir::Direction::Output,
                calyx_ir::Attributes::default()
            ));
        }
        self.builder.register_component(label.name.mangle().clone(), comp_ports);
    }

    /// A component for a call to `call` instantiated as a cell a single time in
    /// the current component.
    fn cell_for_call(
        &mut self, component: &'a mut CalyxComponent<'a, FunctionContext>,
        call: &LabelName, unique: bool
    ) -> (String, RRC<calyx_ir::Cell>) {
        component.
        // For some reason, this is private https://github.com/calyxir/calyx/blob/main/calyx-ir/src/builder.rs#L361
        fn cell_from_signature(
            name: calyx_ir::Id, typ: calyx_ir::CellType,
            ports: Vec<calyx_ir::PortDef<u64>>
        ) -> RRC<calyx_ir::Cell> {
            let cell = calyx_ir::rrc(calyx_ir::Cell::new(name, typ));
            ports.into_iter().for_each(|pd| {
                let port = calyx_ir::rrc(calyx_ir::Port {
                    name: pd.name(),
                    width: pd.width,
                    direction: pd.direction,
                    parent: calyx_ir::PortParent::Cell(calyx_ir::WRC::from(
                        &cell
                    )),
                    attributes: pd.attributes
                });
                cell.borrow_mut().ports.push(port);
            });
            cell
        }

        let cell_name = format!("call{}", call.mangle());
        if let Some(cell) =
            self.call_env.find(cell_name.clone()).filter(|_| !unique)
        {
            (cell_name, cell.clone())
        } else {
            let mut port_defs = self.sig.get(call.mangle()).unwrap().clone();
            let mut go_attr = calyx_ir::Attributes::default();
            go_attr.insert(calyx_ir::Attribute::Num(calyx_ir::NumAttr::Go), 1);
            let mut done_attr = calyx_ir::Attributes::default();
            done_attr
                .insert(calyx_ir::Attribute::Num(calyx_ir::NumAttr::Done), 1);
            let mut clk_attr = calyx_ir::Attributes::default();
            clk_attr
                .insert(calyx_ir::Attribute::Bool(calyx_ir::BoolAttr::Clk), 1);
            let mut reset_attr = calyx_ir::Attributes::default();
            reset_attr.insert(
                calyx_ir::Attribute::Bool(calyx_ir::BoolAttr::Reset),
                1
            );
            port_defs.push(calyx_ir::PortDef::new(
                "go",
                1,
                calyx_ir::Direction::Input,
                go_attr
            ));
            port_defs.push(calyx_ir::PortDef::new(
                "done",
                1,
                calyx_ir::Direction::Output,
                done_attr
            ));
            port_defs.push(calyx_ir::PortDef::new(
                "clk",
                1,
                calyx_ir::Direction::Input,
                clk_attr
            ));
            port_defs.push(calyx_ir::PortDef::new(
                "reset",
                1,
                calyx_ir::Direction::Input,
                reset_attr
            ));
            let cell = cell_from_signature(
                cell_name.clone().into(),
                calyx_ir::CellType::Component {
                    name: call.mangle().clone().into()
                },
                port_defs
            );

            builder.component.cells.add(cell.clone());
            self.call_env.bind(cell_name.clone(), cell.clone());
            (cell_name, cell)
        }
    }

    /// A unique cell that is only used for a single instruction and does not
    /// need to be referenced elsewhere.
    fn cell_for_temp(
        &mut self, builder: &mut calyx_ir::Builder
    ) -> RRC<calyx_ir::Cell> {
        let i = self.temp_count;
        self.temp_count += 1;
        builder.add_primitive(format!("t{}", i), "std_reg", &vec![64])
    }

    fn emit_ir(
        &mut self, component: &mut CalyxComponent<FunctionContext>,
        ir: &Ir
    )  {
        match ir {
            Ir::Add(result, lhs, rhs) => {
                let lhs_cell = self.cell_for_operand(component, lhs);
                let rhs_cell = self.cell_for_operand(component, rhs);
                let result_cell = self.cell_for_var(component, *result);
                let signal_out = component.constant(1, 1);
                let adder =
                    component.named_prim("adder", "std_add", vec![64]);
                let add_group = component.add_group("add");
                let assignments = build_assignments!(builder;
                    adder["left"] = ? lhs_cell["out"];
                    adder["right"] = ? rhs_cell["out"];
                    result_cell["in"] = ? adder["out"];
                    result_cell["write_en"] = ? signal_out["out"];
                    add_group["done"] = ? result_cell["done"];
                )
                .to_vec();
                add_group.borrow_mut().assignments.extend(assignments);
                seq.stmts.push(calyx_ir::Control::enable(add_group));
            }
            Ir::Mul(result, lhs, rhs) => {
                let lhs_cell = self.cell_for_operand(builder, lhs);
                let rhs_cell = self.cell_for_operand(builder, rhs);
                let result_cell = self.cell_for_var(builder, *result);
                let signal_out = builder.add_constant(1, 1);
                let mult =
                    builder.add_primitive("mult", "std_mult_pipe", &vec![64]);

                let mult_group = builder.add_group("multiply");
                let mult_assignments = build_assignments!(builder;
                    mult["left"] = ? lhs_cell["out"];
                    mult["right"] = ? rhs_cell["out"];
                    mult["go"] = ? signal_out["out"];
                    result_cell["in"] = ? mult["out"];
                    result_cell["write_en"] = ? mult["done"];
                    mult_group["done"] = ? result_cell["done"];
                )
                .to_vec();
                mult_group.borrow_mut().assignments.extend(mult_assignments);
                seq.stmts.push(calyx_ir::Control::enable(mult_group));
            }
            Ir::Assign(result, value) => {
                let value_cell = self.cell_for_operand(builder, value);
                if value_cell.borrow().is_primitive("comb_mem_d1".into()) {
                    self.env.bind(*result, value_cell.clone());
                    return;
                }
                let result_cell = self.cell_for_var(builder, *result);
                let signal_out = builder.add_constant(1, 1);
                let assign_group = builder.add_group("assign");
                let assignments = build_assignments!(builder;
                    result_cell["in"] = ? value_cell["out"];
                    result_cell["write_en"] = ? signal_out["out"];
                    assign_group["done"] = ? result_cell["done"];
                )
                .to_vec();
                assign_group.borrow_mut().assignments.extend(assignments);
                seq.stmts.push(calyx_ir::Control::enable(assign_group));
            }
            Ir::GetParam(result) => {
                let func = builder.component.signature.clone();
                let result_cell = self.cell_for_var(builder, *result);
                let signal_out = builder.add_constant(1, 1);
                let get_param_group = builder.add_group("get_param");
                let assignments = build_assignments!(builder;
                    result_cell["in"] = ? func[format!("arg{}", self.param_env)];
                    result_cell["write_en"] = ? signal_out["out"];
                    get_param_group["done"] = ? result_cell["done"];
                )
                .to_vec();
                get_param_group.borrow_mut().assignments.extend(assignments);
                seq.stmts.push(calyx_ir::Control::enable(get_param_group));
                self.param_env += 1;
            }
            Ir::Return(value_opt) => {
                // TODO: handle generating if/else control to simulate early
                // returns, this requires structured IR anyways so doesn't
                // matter right now
                if let Some(value) = value_opt {
                    let return_group = builder.add_group("return");
                    let mut value_cell = self.cell_for_operand(builder, value);

                    // We need to use the done port (doesn't exist on constants)
                    // so if it's a constant we need to make
                    // a temporary port
                    if let Operand::Constant(_) = value {
                        let temp_cell = self.cell_for_temp(builder);
                        return_group.borrow_mut().assignments.extend(
                            build_assignments!(builder;
                                temp_cell["in"] = ? value_cell["out"];
                            )
                        );
                        value_cell = temp_cell;
                    }

                    let ret_cell = self.ret_cell.clone().unwrap();
                    let signal_out = builder.add_constant(1, 1);
                    let assignments = build_assignments!(builder;
                        ret_cell["in"] = ? value_cell["out"];
                        ret_cell["write_en"] = ? signal_out["out"];
                        return_group["done"] = ? ret_cell["done"];
                    )
                    .to_vec();
                    return_group.borrow_mut().assignments.extend(assignments);
                    seq.stmts.push(calyx_ir::Control::enable(return_group));
                } else {
                    // todo!("I haven't figured out return fully yet")
                }
            }
            Ir::LocalAlloc(result, size, count) => {
                self.make_cell_for_pointer(builder, *result, *size * 8, *count);
            }
            Ir::Store {
                result,
                value,
                index
            } => {
                let store_group = builder.add_group("store");
                let result_cell = self.cell_for_var(builder, *result);
                let value_cell = self.cell_for_operand(builder, value);
                let index_cell = self.cell_for_operand(builder, index);
                let signal_out = builder.add_constant(1, 1);
                let assignments = build_assignments!(builder;
                    result_cell["addr0"] = ? index_cell["out"];
                    result_cell["write_data"] = ? value_cell["out"];
                    result_cell["write_en"] = ? signal_out["out"];
                    store_group["done"] = ? result_cell["done"];
                )
                .to_vec();
                store_group.borrow_mut().assignments.extend(assignments);
                seq.stmts.push(calyx_ir::Control::enable(store_group));
            }
            Ir::Load {
                result,
                value,
                index
            } => {
                let load_group = builder.add_group("load");
                let result_cell = self.cell_for_var(builder, *result);
                let value_cell = self.cell_for_operand(builder, value);
                let index_cell = self.cell_for_operand(builder, index);
                let signal_out = builder.add_constant(1, 1);
                let assignments = build_assignments!(builder;
                    value_cell["addr0"] = ? index_cell["out"];
                    result_cell["in"] = ? value_cell["read_data"];
                    result_cell["write_en"] = ? signal_out["out"];
                    load_group["done"] = ? result_cell["done"];
                )
                .to_vec();
                load_group.borrow_mut().assignments.extend(assignments);
                seq.stmts.push(calyx_ir::Control::enable(load_group));
            }
            Ir::Map {
                result,
                parallel_factor,
                f,
                input,
                length
            } => {
                assert!(length % parallel_factor == 0, "parallel_factor must divide length. figure out a better place to assert this, probably in the type checker fix");
                let index_cell = self.cell_for_temp(builder);
                let signal_out = builder.add_constant(1, 1);

                let init_group = builder.add_group("init");
                let zero = builder.add_constant(0, 64);
                let init_assignments = build_assignments!(builder;
                    index_cell["in"] = ? zero["out"];
                    index_cell["write_en"] = ? signal_out["out"];
                    init_group["done"] = ? index_cell["done"];
                )
                .to_vec();
                init_group.borrow_mut().assignments.extend(init_assignments);

                let cond_group = builder.add_comb_group("cond");
                let array_size_cell = builder.add_constant(*length as u64, 64);
                let lt_cell = builder.add_primitive("lt", "std_lt", &vec![64]);
                let cond_assignments = build_assignments!(builder;
                    lt_cell["left"] = ? index_cell["out"];
                    lt_cell["right"] = ? array_size_cell["out"];
                )
                .to_vec();
                cond_group.borrow_mut().assignments.extend(cond_assignments);

                let read_group = builder.add_group("read");
                let write_group = builder.add_group("write");

                let input_cell = self.cell_for_operand(builder, input); // also a memory
                let result_cell = self.cell_for_var(builder, *result);
                let (_, call_cell) = self.cell_for_call(builder, f, true);

                let read_assignments = build_assignments!(builder;
                    input_cell["addr0"] = ? index_cell["out"];
                    call_cell["arg0"] = ? input_cell["read_data"];
                    call_cell["go"] = ? signal_out["out"];
                    read_group["done"] = ? call_cell["done"];
                )
                .to_vec();
                read_group.borrow_mut().assignments.extend(read_assignments);

                let write_assignments = build_assignments!(builder;
                    result_cell["addr0"] = ? index_cell["out"];
                    result_cell["write_data"] = ? call_cell["ret"];
                    result_cell["write_en"] = ? call_cell["done"];
                    write_group["done"] = ? result_cell["done"];
                )
                .to_vec();
                write_group
                    .borrow_mut()
                    .assignments
                    .extend(write_assignments);

                let incr_group = builder.add_group("incr");
                let adder =
                    builder.add_primitive("adder", "std_add", &vec![64]);
                let one = builder.add_constant(1, 64);
                let incr_assignments = build_assignments!(builder;
                    adder["left"] = ? index_cell["out"];
                    adder["right"] = ? one["out"];
                    index_cell["in"] = ? adder["out"];
                    index_cell["write_en"] = ? signal_out["out"];
                    incr_group["done"] = ? index_cell["done"];
                )
                .to_vec();
                incr_group.borrow_mut().assignments.extend(incr_assignments);

                seq.stmts.push(calyx_ir::Control::enable(init_group));
                seq.stmts.push(calyx_ir::Control::while_(
                    lt_cell.borrow().get("out"),
                    Some(cond_group),
                    Box::new(calyx_ir::Control::seq(vec![
                        calyx_ir::Control::enable(read_group),
                        calyx_ir::Control::enable(write_group),
                        calyx_ir::Control::enable(incr_group),
                    ]))
                ));
            }
            Ir::Call(result_opt, func_name, args) => {
                let (_, call_cell) =
                    self.cell_for_call(builder, func_name, false);
                let signal_out = builder.add_constant(1, 1);
                let call_group = builder.add_group("call");
                for (i, arg) in args.iter().enumerate() {
                    let arg_port =
                        self.cell_for_operand(builder, arg).borrow().get("out");
                    let assignment = builder.build_assignment(
                        call_cell.borrow().get(format!("arg{}", i)),
                        arg_port,
                        calyx_ir::Guard::True
                    );
                    call_group.borrow_mut().assignments.push(assignment);
                }
                let further_assignments = build_assignments!(builder;
                    call_cell["go"] = ? signal_out["out"];
                    call_group["done"] = ? call_cell["done"];
                );
                call_group
                    .borrow_mut()
                    .assignments
                    .extend(further_assignments);
                seq.stmts.push(calyx_ir::Control::enable(call_group));

                if let Some(result) = result_opt {
                    let use_call_group = builder.add_group("use_call");
                    let result_cell = self.cell_for_var(builder, *result);
                    let signal_out = builder.add_constant(1, 1);
                    let use_assignments = build_assignments!(builder;
                        result_cell["in"] = ? call_cell["ret"];
                        result_cell["write_en"] = ? signal_out["out"];
                        use_call_group["done"] = ? result_cell["done"];
                    );
                    use_call_group
                        .borrow_mut()
                        .assignments
                        .extend(use_assignments);
                    seq.stmts.push(calyx_ir::Control::enable(use_call_group));
                }
            }
        }
    }

    fn emit_block(
        &mut self, mut component: &mut CalyxComponent<FunctionContext>,
        block: BasicBlockCell
    ) {
        for ir in block.as_ref().into_iter() {
            self.emit_ir(&mut component, ir);
        }
    }

    fn emit_func(
        &mut self, label: &Label, _args: &Vec<Type>, ret: &Box<Type>,
        _is_pure: bool, cfg: &ControlFlowGraph
    ) {
        let mut component =
            self.builder.start_component(label.name.mangle().clone());

        if **ret != Type::Unit {
            let func = component.signature();
            let ret_cell =
                component.unnamed_cell(builder::CalyxCellKind::Register {
                    size: ret.size() * 8
                });
            component.user_data_mut().ret_cell = Some(ret_cell.clone());
            let always = 
                build_assignments_2!(component;
                    func["ret"] = ? ret_cell["out"];
                )
                .to_vec();
            component.with_calyx_builder(|builder| {
                builder.add_continuous_assignments(always);
            });
        }

        // for block in cfg.blocks() {
        //     self.emit_block(block);
        // }
        assert_eq!(1, cfg.size(), "CalyxBackend requires structured IR only in the entry block, but other blocks were found in the CFG");
        self.emit_block(&mut component, cfg.entry());
    }
}

pub struct CalyxBackendInput {
    pub lib_path: PathBuf
}

impl<'a> PulsarBackend for CalyxBackend<'a> {
    type InitInput = CalyxBackendInput;
    type Error = calyx_utils::Error;

    fn new(input: Self::InitInput) -> Self {
        let mut prelude_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        prelude_file_path.push("resources");
        prelude_file_path.push("prelude.futil");

        Self {
            builder: CalyxBuilder::new(
                Some(prelude_file_path),
                input.lib_path,
                "main".into(),
                "_".into()
            )
        }
    }

    fn run(
        &mut self, code: Vec<GeneratedTopLevel>, output: Output
    ) -> Result<(), Self::Error> {
        // Create a calyx program from the IR
        // - Step 1: load signatures
        for generated_top_level in &code {
            match generated_top_level {
                GeneratedTopLevel::Function {
                    label,
                    args,
                    ret,
                    is_pure: _,
                    cfg: _
                } => self.cache_func_sig(label, args, ret)
            }
        }
        // - Step 2: emit generated IR
        for generated_top_level in &code {
            match generated_top_level {
                GeneratedTopLevel::Function {
                    label,
                    args,
                    ret,
                    is_pure,
                    cfg
                } => self.emit_func(label, args, ret, *is_pure, cfg)
            }
        }

        // Obtain the program context
        let mut builder = CalyxBuilder::dummy();
        std::mem::swap(&mut self.builder, &mut builder);
        let mut calyx_ctx = self.builder.finalize();

        // Debug print
        calyx_ir::Printer::write_context(&calyx_ctx, false, &mut stderr())
            .unwrap();

        // Perform optimization passes
        let pm = calyx_opt::pass_manager::PassManager::default_passes()?;
        let backend_conf = calyx_ir::BackendConf {
            synthesis_mode: false,
            enable_verification: false,
            flat_assign: true,
            emit_primitive_extmodules: false
        };
        calyx_ctx.bc = backend_conf;
        pm.execute_plan(
            &mut calyx_ctx,
            &["all".to_string()],
            &["canonicalize".to_string()],
            false
        )?;

        // Emit to Verilog
        let backend = calyx_backend::VerilogBackend;
        backend.run(calyx_ctx, output.into())?;
        Ok(())
    }
}

impl Into<calyx_utils::OutputFile> for Output {
    fn into(self) -> calyx_utils::OutputFile {
        match self {
            Output::Stdout => calyx_utils::OutputFile::Stdout,
            Output::Stderr => calyx_utils::OutputFile::Stderr,
            Output::File(path) => calyx_utils::OutputFile::File(path)
        }
    }
}
