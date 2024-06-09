// Copyright (C) 2024 Ethan Uppal. All rights reserved.
use super::{
    basic_block::BasicBlockCell,
    control_flow_graph::ControlFlowGraph,
    label::{Label, LabelName, LabelVisibility},
    operand::Operand,
    variable::Variable,
    Ir
};
use pulsar_frontend::{
    ast::{Expr, ExprValue, Node, NodeValue, Param},
    token::{Token, TokenType},
    ty::Type
};
use pulsar_utils::environment::Environment;
use std::fmt::Display;

pub enum GeneratedTopLevel {
    Function {
        label: Label,
        args: Vec<Type>,
        ret: Box<Type>,
        is_pure: bool,
        cfg: ControlFlowGraph
    }
}

impl Display for GeneratedTopLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Function {
                label,
                args,
                ret,
                is_pure,
                cfg
            } => {
                writeln!(
                    f,
                    "{} {}({}) -> {}:",
                    if *is_pure { "pure " } else { "" },
                    label,
                    args.iter()
                        .map(|ty| ty.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    ret
                )?;
                write!(f, "{}", cfg)
            }
        }
    }
}

pub struct Generator {
    program: Box<dyn Iterator<Item = Node>>,
    env: Environment<String, Variable>
}

impl Generator {
    pub fn new(program: Vec<Node>) -> Self {
        Self {
            program: Box::new(program.into_iter()),
            env: Environment::new()
        }
    }

    fn gen_expr(&mut self, expr: &Expr, block: BasicBlockCell) -> Operand {
        match &expr.value {
            ExprValue::ConstantInt(value) => Operand::Constant(*value),
            ExprValue::BinOp(lhs, op, rhs) => {
                let lhs = self.gen_expr(lhs, block.clone());
                let rhs = self.gen_expr(rhs, block.clone());
                let result = Variable::new();
                match op.ty {
                    TokenType::Plus => {
                        block.as_mut().add(Ir::Add(result, lhs, rhs))
                    }
                    TokenType::Times => {
                        block.as_mut().add(Ir::Mul(result, lhs, rhs))
                    }
                    _ => todo!()
                }
                Operand::Variable(result)
            }
            ExprValue::BoundName(name) => {
                Operand::Variable(*self.env.find(name.value.clone()).unwrap())
            }
            ExprValue::Call(name, args) => {
                let mut arg_operands = vec![];
                let mut arg_tys = vec![];
                for arg in args {
                    arg_tys.push(arg.ty.clone_out());
                    let arg_operand = self.gen_expr(arg, block.clone());
                    arg_operands.push(arg_operand);
                }
                let result_opt = if expr.ty.clone_out() == Type::Unit {
                    None
                } else {
                    Some(Variable::new())
                };
                block.as_mut().add(Ir::Call(
                    result_opt,
                    LabelName::from_native(
                        name.value.clone(),
                        &arg_tys,
                        &Box::new(expr.ty.clone_out())
                    ),
                    arg_operands
                ));
                result_opt.map_or(Operand::Constant(0), |result| {
                    Operand::Variable(result)
                })
            }
            ExprValue::Subscript(array, index) => {
                let array_operand = self.gen_expr(array, block.clone());
                let index_operand = self.gen_expr(index, block.clone());
                let result = Variable::new();
                block.as_mut().add(Ir::Load {
                    result,
                    value: array_operand,
                    index: index_operand
                });
                Operand::Variable(result)
            }
            ExprValue::ArrayLiteral(elements, _) => {
                let (element_type, element_count) =
                    expr.ty.as_ref().as_array_type();
                let element_size = element_type.as_ref().size();
                let element_count = element_count as usize;

                let result = Variable::new();
                block.as_mut().add(Ir::LocalAlloc(
                    result,
                    element_size,
                    element_count
                ));
                for (i, element) in elements.iter().enumerate() {
                    let element_operand = self.gen_expr(element, block.clone());
                    block.as_mut().add(Ir::Store {
                        result,
                        value: element_operand,
                        index: Operand::Constant(i as i64)
                    });
                }
                for i in elements.len()..element_count {
                    block.as_mut().add(Ir::Store {
                        result,
                        value: Operand::Constant(0),
                        index: Operand::Constant(i as i64)
                    });
                }
                Operand::Variable(result)
            }
            ExprValue::HardwareMap(_, parallel_factor, f, arr) => {
                let (element_type, element_count) =
                    expr.ty.as_ref().as_array_type();
                let element_size = element_type.as_ref().size();
                let element_count = element_count as usize;

                let arr_operand = self.gen_expr(arr, block.clone());

                let result = Variable::new();
                block.as_mut().add(Ir::LocalAlloc(
                    result,
                    element_size,
                    element_count
                ));

                block.as_mut().add(Ir::Map {
                    result,
                    parallel_factor: *parallel_factor,
                    f: LabelName::from_native(
                        f.value.clone(),
                        &vec![Type::Int64],
                        &Box::new(Type::Int64)
                    ),
                    input: arr_operand,
                    length: element_count
                });

                Operand::Variable(result)
            }
            _ => todo!()
        }
    }

    fn gen_node(&mut self, node: &Node, block: BasicBlockCell) {
        match &node.value {
            NodeValue::LetBinding {
                name,
                hint: _,
                value
            } => {
                let value_operand = self.gen_expr(value, block.clone());
                let name_var = Variable::new();
                self.env.bind(name.value.clone(), name_var);
                block.as_mut().add(Ir::Assign(name_var, value_operand));
            }
            NodeValue::Return {
                keyword_token: _,
                value
            } => {
                let value_operand = value
                    .as_ref()
                    .map(|value| self.gen_expr(value.as_ref(), block.clone()));
                block.as_mut().add(Ir::Return(value_operand));
            }
            _ => {}
        }
    }

    fn gen_func(
        &mut self, name: &Token, params: &Vec<Param>, ret: Type, is_pure: bool,
        body: &Vec<Node>
    ) -> GeneratedTopLevel {
        self.env.push();
        let cfg = ControlFlowGraph::new();
        let entry = cfg.entry();
        for (name, _) in params {
            let param_var = Variable::new();
            self.env.bind(name.value.clone(), param_var);
            entry.as_mut().add(Ir::GetParam(param_var));
        }
        for node in body {
            self.gen_node(node, entry.clone());
        }
        self.env.pop();
        let arg_tys = params.iter().map(|(_, ty)| ty.clone()).collect();
        let ret_ty = Box::new(ret);
        GeneratedTopLevel::Function {
            label: Label::from(
                LabelName::from_native(name.value.clone(), &arg_tys, &ret_ty),
                LabelVisibility::Private
            ),
            args: arg_tys,
            ret: ret_ty,
            is_pure,
            cfg
        }
    }
}

impl Iterator for Generator {
    type Item = GeneratedTopLevel;

    fn next(&mut self) -> Option<GeneratedTopLevel> {
        match self.program.next()?.value {
            NodeValue::Function {
                name,
                params,
                ret,
                pure_token,
                body
            } => Some(self.gen_func(
                &name,
                &params,
                ret,
                pure_token.is_some(),
                &body
            )),
            _ => None
        }
    }
}
