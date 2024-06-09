// Copyright (C) 2024 Ethan Uppal. All rights reserved.

use super::PulsarBackend;
use crate::{build_assignments_2, finish_component, Output};
use builder::{
    CalyxAssignmentContainer, CalyxBuilder, CalyxCell, CalyxCellKind,
    CalyxComponent, CalyxControl, Sequential
};
use calyx_backend::Backend;
use pulsar_frontend::ty::Type;
use pulsar_ir::{
    basic_block::BasicBlockCell,
    control_flow_graph::ControlFlowGraph,
    generator::GeneratedTopLevel,
    label::{Label, LabelName, MAIN_SYMBOL_PREFIX},
    operand::Operand,
    variable::Variable,
    Ir
};
use std::{io::stderr, path::PathBuf};

pub mod builder;

// This file contains many examples of BAD software engineering.
// All components are treated very much like functions. They have input ports
// all of width 64 and one output port of width 64. However hardware is a lot
// more flexible than this. See if you can figure out how to better use it.
//
// I realized that a contributing factor to this is that my IR has everything
// has Int64. I should change that

#[derive(Default)]
struct FunctionContext {
    ret_cell: Option<CalyxCell>,
    param_env: usize
}

pub struct CalyxBackend {
    builder: CalyxBuilder
}

impl CalyxBackend {
    // TODO: support larger nested arrays
    // but arbitrary pointer access needs to be restricted in static analyzer
    // when targeting hardware
    fn make_cell_for_array(
        &self, component: &mut CalyxComponent<FunctionContext>, var: Variable,
        cell_size: usize, length: usize
    ) -> CalyxCell {
        component.named_mem(var.to_string(), cell_size, length, 64)
    }

    /// Builds a constant if the operand is a constant and looks up a variable
    /// otherwise.
    fn find_operand_cell(
        &self, component: &mut CalyxComponent<FunctionContext>,
        operand: &Operand
    ) -> CalyxCell {
        match &operand {
            Operand::Constant(value) => component.constant(*value, 64),
            Operand::Variable(var) => component.find(var.to_string())
        }
    }

    fn register_func(
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
        self.builder
            .register_component(label.name.mangle().clone(), comp_ports);

        if label.name.mangle().starts_with(MAIN_SYMBOL_PREFIX) {
            self.builder.set_entrypoint(label.name.mangle().clone());
        }
    }

    /// A component for a call to `call` instantiated as a cell a single time in
    /// the current component if `unique` and instantiated fresh otherwise.
    fn cell_for_call(
        &self, component: &mut CalyxComponent<FunctionContext>,
        call: &LabelName, unique: bool
    ) -> (String, CalyxCell) {
        let callee_name = call.mangle().clone();
        let cell_name = format!("call{}", callee_name);
        component.component_cell(cell_name, callee_name, unique)
    }

    /// A unique cell that is only used for a single instruction and does not
    /// need to be referenced elsewhere.
    fn new_unnamed_reg(
        &self, component: &mut CalyxComponent<FunctionContext>
    ) -> CalyxCell {
        component.new_unnamed_cell(CalyxCellKind::Register { size: 64 })
    }

    fn emit_ir(
        &self, component: &mut CalyxComponent<FunctionContext>,
        parent: &mut CalyxControl<Sequential>, ir: &Ir
    ) {
        let signal_out = component.signal_out();
        match ir {
            Ir::Add(result, lhs, rhs) => {
                let lhs_cell = self.find_operand_cell(component, lhs);
                let rhs_cell = self.find_operand_cell(component, rhs);
                let result_cell = component.new_reg(result.to_string(), 64);
                let adder = component.new_prim("adder", "std_add", vec![64]);
                let add_group = component.add_group("add");
                add_group.extend(build_assignments_2!(component;
                    adder["left"] = ? lhs_cell["out"];
                    adder["right"] = ? rhs_cell["out"];
                    result_cell["in"] = ? adder["out"];
                    result_cell["write_en"] = ? signal_out["out"];
                    add_group["done"] = ? result_cell["done"];
                ));
                parent.enable_next(&add_group);
            }
            Ir::Mul(result, lhs, rhs) => {
                let lhs_cell = self.find_operand_cell(component, lhs);
                let rhs_cell = self.find_operand_cell(component, rhs);
                let result_cell = component.new_reg(result.to_string(), 64);
                let mult =
                    component.new_prim("mult", "std_mult_pipe", vec![64]);
                let mult_group = component.add_group("multiply");
                mult_group.extend(build_assignments_2!(component;
                    mult["left"] = ? lhs_cell["out"];
                    mult["right"] = ? rhs_cell["out"];
                    mult["go"] = ? signal_out["out"];
                    result_cell["in"] = ? mult["out"];
                    result_cell["write_en"] = ? mult["done"];
                    mult_group["done"] = ? result_cell["done"];
                ));
                parent.enable_next(&mult_group);
            }
            Ir::Assign(result, value) => {
                let value_cell = self.find_operand_cell(component, value);
                if value_cell.kind.is_memory() {
                    // "copy" pointer
                    component.alias_cell(result.to_string(), value_cell);
                    return;
                }
                let result_cell = component.new_reg(result.to_string(), 64);
                let assign_group = component.add_group("assign");
                assign_group.extend(build_assignments_2!(component;
                    result_cell["in"] = ? value_cell["out"];
                    result_cell["write_en"] = ? signal_out["out"];
                    assign_group["done"] = ? result_cell["done"];
                ));
                parent.enable_next(&assign_group);
            }
            Ir::GetParam(result) => {
                let func = component.signature();
                // TODO: memory refs
                let result_cell = component.new_reg(result.to_string(), 64);
                let get_param_group = component.add_group("get_param");
                let param_port =
                    format!("arg{}", component.user_data_ref().param_env);
                get_param_group.extend(build_assignments_2!(component;
                    result_cell["in"] = ? func[param_port];
                    result_cell["write_en"] = ? signal_out["out"];
                    get_param_group["done"] = ? result_cell["done"];
                ));
                parent.enable_next(&get_param_group);
                component.user_data_mut().param_env += 1;
            }
            Ir::Return(value_opt) => {
                // TODO: handle generating if/else control to simulate early
                // returns, this requires structured IR anyways so doesn't
                // matter right now
                if let Some(value) = value_opt {
                    let return_group = component.add_group("return");
                    let mut value_cell =
                        self.find_operand_cell(component, value);

                    // We need to use the done port (doesn't exist on constants)
                    // so if it's a constant we need to make
                    // a temporary port
                    if let Operand::Constant(_) = value {
                        let temp_cell = self.new_unnamed_reg(component);
                        return_group.extend(build_assignments_2!(component;
                            temp_cell["in"] = ? value_cell["out"];
                        ));
                        value_cell = temp_cell;
                    }

                    let ret_cell = component
                        .user_data_ref()
                        .ret_cell
                        .as_ref()
                        .cloned()
                        .unwrap();
                    return_group.extend(build_assignments_2!(component;
                        ret_cell["in"] = ? value_cell["out"];
                        ret_cell["write_en"] = ? signal_out["out"];
                        return_group["done"] = ? ret_cell["done"];
                    ));
                    parent.enable_next(&return_group);
                } else {
                    // todo!("I haven't figured out return fully yet")
                }
            }
            Ir::LocalAlloc(result, size, count) => {
                self.make_cell_for_array(component, *result, *size * 8, *count);
            }
            Ir::Store {
                result,
                value,
                index
            } => {
                let store_group = component.add_group("store");
                let result_cell = component.find(result.to_string());
                let value_cell = self.find_operand_cell(component, value);
                let index_cell = self.find_operand_cell(component, index);
                assert!(
                    result_cell.kind.is_memory(),
                    "Ir::Store should take a memory result cell"
                );
                store_group.extend(build_assignments_2!(component;
                    result_cell["addr0"] = ? index_cell["out"];
                    result_cell["write_data"] = ? value_cell["out"];
                    result_cell["write_en"] = ? signal_out["out"];
                    store_group["done"] = ? result_cell["done"];
                ));
                parent.enable_next(&store_group);
            }
            Ir::Load {
                result,
                value,
                index
            } => {
                let load_group = component.add_group("load");
                let result_cell = component.new_reg(result.to_string(), 64);
                let value_cell = self.find_operand_cell(component, value);
                assert!(
                    value_cell.kind.is_memory(),
                    "Ir::Load should take a memory result cell"
                );
                let index_cell = self.find_operand_cell(component, index);
                load_group.extend(build_assignments_2!(component;
                    value_cell["addr0"] = ? index_cell["out"];
                    result_cell["in"] = ? value_cell["read_data"];
                    result_cell["write_en"] = ? signal_out["out"];
                    load_group["done"] = ? result_cell["done"];
                ));
                parent.enable_next(&load_group);
            }
            Ir::Map {
                result,
                parallel_factor,
                f,
                input,
                length
            } => {
                assert!(length % parallel_factor == 0, "parallel_factor must divide length. figure out a better place to assert this, probably in the type checker fix");
                let index_cell = self.new_unnamed_reg(component);

                let init_group = component.add_group("init");
                let zero = component.constant(0, 64);
                init_group.extend(build_assignments_2!(component;
                    index_cell["in"] = ? zero["out"];
                    index_cell["write_en"] = ? signal_out["out"];
                    init_group["done"] = ? index_cell["done"];
                ));

                let cond_group = component.add_comb_group("cond");
                let array_size_cell = component.constant(*length as i64, 64);
                let lt_cell = component.new_prim("lt", "std_lt", vec![64]);
                cond_group.extend(build_assignments_2!(component;
                    lt_cell["left"] = ? index_cell["out"];
                    lt_cell["right"] = ? array_size_cell["out"];
                ));

                let read_group = component.add_group("read");
                let write_group = component.add_group("write");

                let input_cell = self.find_operand_cell(component, input); // also a memory
                assert!(
                    input_cell.kind.is_memory(),
                    "Ir::Map should take a memory input cell"
                );
                let result_cell = component.find(result.to_string());
                assert!(
                    result_cell.kind.is_memory(),
                    "Ir::Map should take a memory result cell"
                );
                let (_, call_cell) = self.cell_for_call(component, f, true);

                read_group.extend(build_assignments_2!(component;
                    input_cell["addr0"] = ? index_cell["out"];
                    call_cell["arg0"] = ? input_cell["read_data"];
                    call_cell["go"] = ? signal_out["out"];
                    read_group["done"] = ? call_cell["done"];
                ));

                write_group.extend(build_assignments_2!(component;
                    result_cell["addr0"] = ? index_cell["out"];
                    result_cell["write_data"] = ? call_cell["ret"];
                    result_cell["write_en"] = ? call_cell["done"];
                    write_group["done"] = ? result_cell["done"];
                ));

                let incr_group = component.add_group("incr");
                let adder = component.new_prim("adder", "std_add", vec![64]);
                let one = component.constant(1, 64);
                incr_group.extend(build_assignments_2!(component;
                    adder["left"] = ? index_cell["out"];
                    adder["right"] = ? one["out"];
                    index_cell["in"] = ? adder["out"];
                    index_cell["write_en"] = ? signal_out["out"];
                    incr_group["done"] = ? index_cell["done"];
                ));

                parent.enable_next(&init_group);
                parent.while_(lt_cell.get("out"), Some(cond_group), |s| {
                    s.enable_next(&read_group);
                    s.enable_next(&write_group);
                    s.enable_next(&incr_group);
                });
            }
            Ir::Call(result_opt, func_name, args) => {
                let (_, call_cell) =
                    self.cell_for_call(component, func_name, false);
                let call_group = component.add_group("call");
                for (i, arg) in args.iter().enumerate() {
                    let arg_port =
                        self.find_operand_cell(component, arg).get("out");
                    call_group.add(component.with_calyx_builder(|b| {
                        b.build_assignment(
                            call_cell.get(&format!("arg{}", i)),
                            arg_port,
                            calyx_ir::Guard::True
                        )
                    }));
                }
                call_group.extend(build_assignments_2!(component;
                    call_cell["go"] = ? signal_out["out"];
                    call_group["done"] = ? call_cell["done"];
                ));
                parent.enable_next(&call_group);

                if let Some(result) = result_opt {
                    let use_call_group = component.add_group("use_call");
                    let result_cell = component.new_reg(result.to_string(), 64);
                    use_call_group.extend(build_assignments_2!(component;
                        result_cell["in"] = ? call_cell["ret"];
                        result_cell["write_en"] = ? signal_out["out"];
                        use_call_group["done"] = ? result_cell["done"];
                    ));
                    parent.enable_next(&use_call_group);
                }
            }
        }
    }

    fn emit_block(
        &self, mut component: &mut CalyxComponent<FunctionContext>,
        parent: &mut CalyxControl<Sequential>, block: BasicBlockCell
    ) {
        parent.seq(|s| {
            for ir in block.as_ref().into_iter() {
                self.emit_ir(&mut component, s, ir);
            }
        });
    }

    fn emit_func(
        &mut self, label: &Label, _args: &Vec<Type>, ret: &Box<Type>,
        _is_pure: bool, cfg: &ControlFlowGraph
    ) {
        let mut component: CalyxComponent<FunctionContext> =
            self.builder.start_component(label.name.mangle().clone());

        if **ret != Type::Unit {
            let func = component.signature();
            let ret_cell =
                component.new_unnamed_cell(builder::CalyxCellKind::Register {
                    size: ret.size() * 8
                });
            component.user_data_mut().ret_cell = Some(ret_cell.clone());
            let always = build_assignments_2!(component;
                func["ret"] = ? ret_cell["out"];
            )
            .to_vec();
            component.with_calyx_builder(|b| {
                b.add_continuous_assignments(always);
            });
        }

        // for block in cfg.blocks() {
        //     self.emit_block(block);
        // }
        assert_eq!(1, cfg.size(), "CalyxBackend requires structured IR only in the entry block, but other blocks were found in the CFG");
        let mut root_control = CalyxControl::default();
        self.emit_block(&mut component, &mut root_control, cfg.entry());
        *component.control() = root_control;

        finish_component!(self.builder, component);
    }
}

pub struct CalyxBackendInput {
    pub lib_path: PathBuf
}

impl PulsarBackend for CalyxBackend {
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
                None,
                "_".into()
            )
        }
    }

    fn run(
        mut self, code: Vec<GeneratedTopLevel>, output: Output
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
                } => {
                    self.register_func(label, args, ret);
                }
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
        std::mem::swap(&mut builder, &mut self.builder);
        let mut calyx_ctx = builder.finalize();

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

impl From<Output> for calyx_utils::OutputFile {
    fn from(output: Output) -> Self {
        match output {
            Output::Stdout => calyx_utils::OutputFile::Stdout,
            Output::Stderr => calyx_utils::OutputFile::Stderr,
            Output::File(path) => calyx_utils::OutputFile::File(path)
        }
    }
}
