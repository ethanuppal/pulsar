use super::PulsarBackend;
use crate::{
    frontend::ty::Type,
    ir::{
        basic_block::BasicBlockCell, control_flow_graph::ControlFlowGraph,
        generator::GeneratedTopLevel, label::Label, operand::Operand,
        variable::Variable, Ir
    },
    utils::environment::Environment
};
use calyx_backend::Backend;
use calyx_ir::{build_assignments, RRC};

// This file contains many examples of BAD software engineering.

pub struct CalyxBackend {
    env: Environment<Variable, RRC<calyx_ir::Cell>>,
    param_env: usize
}

impl CalyxBackend {
    fn stdlib_context() -> calyx_ir::Context {
        let ws = calyx_frontend::Workspace::from_compile_lib().unwrap();
        calyx_ir::Context {
            components: vec![],
            lib: ws.lib,
            entrypoint: "main".into(),
            bc: calyx_ir::BackendConf::default(),
            extra_opts: vec![],
            metadata: None
        }
    }

    fn cell_for_var(
        &mut self, builder: &mut calyx_ir::Builder, var: Variable
    ) -> RRC<calyx_ir::Cell> {
        if let Some(cell) = self.env.find(var) {
            cell.clone()
        } else {
            let cell =
                builder.add_primitive(var.to_string(), "std_reg", &vec![64]);
            self.env.bind(var, cell.clone());
            cell
        }
    }

    fn cell_for_operand(
        &mut self, builder: &mut calyx_ir::Builder, operand: &Operand
    ) -> RRC<calyx_ir::Cell> {
        match &operand {
            Operand::Constant(value) => builder.add_constant(*value as u64, 64),
            Operand::Variable(var) => self.cell_for_var(builder, *var)
        }
    }

    fn emit_ir(
        &mut self, builder: &mut calyx_ir::Builder, seq: &mut calyx_ir::Seq,
        ir: &Ir
    ) {
        match ir {
            Ir::Add(result, lhs, rhs) => todo!(),
            Ir::Mul(_, _, _) => todo!(),
            Ir::Assign(_, _) => todo!(),
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
                    let func = builder.component.signature.clone();
                    let value_cell = self.cell_for_operand(builder, value);
                    let return_group = builder.add_group("return");
                    let assignments = build_assignments!(builder;
                        func["ret"] = ? value_cell["out"];
                        return_group["done"] = ? value_cell["done"];
                    )
                    .to_vec();
                    return_group.borrow_mut().assignments.extend(assignments);
                    seq.stmts.push(calyx_ir::Control::enable(return_group));
                }
            }
            Ir::LocalAlloc(_, _) => todo!(),
            Ir::Store {
                result,
                value,
                index
            } => todo!(),
            Ir::Map {
                result,
                parallel_factor,
                f,
                input
            } => todo!(),
            Ir::Call(_, _, _) => todo!()
        }
    }

    fn emit_block(
        &mut self, builder: &mut calyx_ir::Builder, seq: &mut calyx_ir::Seq,
        block: BasicBlockCell
    ) {
        for ir in block.as_ref().into_iter() {
            self.emit_ir(builder, seq, ir);
        }
    }

    fn emit_func(
        &mut self, calyx_ctx: &mut calyx_ir::Context, label: &Label,
        args: &Vec<Type>, ret: &Box<Type>, is_pure: bool,
        cfg: &ControlFlowGraph
    ) {
        let comp_name = calyx_ir::Id::new(label.name.mangle(args, ret));
        let mut comp_ports = vec![];
        for (i, arg) in args.iter().enumerate() {
            let width = arg.size();
            let name = format!("arg{}", i);
            comp_ports.push(calyx_ir::PortDef::new(
                name,
                (width as u64) * 8,
                calyx_ir::Direction::Input,
                calyx_ir::Attributes::default()
            ));
        }
        if **ret != Type::Unit {
            comp_ports.push(calyx_ir::PortDef::new(
                "ret",
                (ret.size() as u64) * 8,
                calyx_ir::Direction::Output,
                calyx_ir::Attributes::default()
            ));
        }

        let mut comp = calyx_ir::Component::new(
            comp_name, comp_ports, true, is_pure, None
        );

        let mut builder =
            calyx_ir::Builder::new(&mut comp, &calyx_ctx.lib).not_generated();

        let mut main_seq = calyx_ir::Seq {
            stmts: vec![],
            attributes: calyx_ir::Attributes::default()
        };

        // for block in cfg.blocks() {
        //     self.emit_block(block);
        // }
        self.param_env = 0;
        self.emit_block(&mut builder, &mut main_seq, cfg.entry());
        *comp.control.borrow_mut() = calyx_ir::Control::Seq(main_seq);

        calyx_ctx.components.push(comp);
    }
}

pub struct CalyxBackendInput {
    pub output_file: calyx_utils::OutputFile
}

impl PulsarBackend for CalyxBackend {
    type ExtraInput = CalyxBackendInput;
    type Error = calyx_utils::Error;

    fn new() -> Self {
        Self {
            env: Environment::new(),
            param_env: 0
        }
    }

    fn run(
        &mut self, code: Vec<GeneratedTopLevel>, input: Self::ExtraInput
    ) -> Result<(), Self::Error> {
        let mut calyx_ctx = CalyxBackend::stdlib_context();

        // Create a calyx program from the IR
        for generated_top_level in &code {
            match generated_top_level {
                GeneratedTopLevel::Function {
                    label,
                    args,
                    ret,
                    is_pure,
                    cfg
                } => self.emit_func(
                    &mut calyx_ctx,
                    label,
                    args,
                    ret,
                    *is_pure,
                    cfg
                )
            }
        }

        // Debug print
        calyx_ir::Printer::write_context(
            &calyx_ctx,
            false,
            &mut std::io::stdout()
        )
        .unwrap();

        calyx_ctx.entrypoint = calyx_ctx
            .components
            .iter()
            .find(|comp| comp.name.to_string().contains("_pulsar_Smain"))
            .unwrap()
            .name;

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
        backend.run(calyx_ctx, input.output_file)?;
        Ok(())
    }
}
