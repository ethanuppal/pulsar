// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

use super::{
    label::{Label, Name, Visibility},
    operand::Operand,
    variable::Variable,
    Ir
};
use crate::control::{Control, Seq};
use pulsar_frontend::{
    ast::{
        expr::{Expr, ExprValue},
        pretty_print::PrettyPrint,
        stmt::Stmt,
        ty::{LiquidTypeValue, Type, TypeValue},
        AsASTPool
    },
    token::{Token, TokenType}
};
use pulsar_utils::{
    environment::Environment,
    id::Gen,
    pool::{AsPool, Handle}
};
use std::fmt::{self, Display, Write};

pub struct GeneratedFunction {
    label: Label,
    args: Vec<Handle<Type>>,
    ret: Handle<Type>,
    cfg: Control
}

impl PrettyPrint for GeneratedFunction {
    fn pretty_print(
        &self, f: &mut inform::fmt::IndentFormatter<'_, '_>
    ) -> core::fmt::Result {
        writeln!(
            f,
            "cfg {}({}) -> {} {{",
            self.label,
            self.args
                .iter()
                .map(|ty| ty.to_string())
                .collect::<Vec<_>>()
                .join(", "),
            self.ret
        )?;
        f.increase_indent();
        self.cfg.pretty_print(f)?;
        f.decrease_indent();
        write!(f, "}}")
    }
}

impl Display for GeneratedFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}

pub trait AsGenerationPool: AsASTPool + AsPool<Control, ()> {}

/// IR generator.
pub struct Generator<'pool, P: AsGenerationPool> {
    ast: Vec<Handle<Stmt>>,
    pool: &'pool mut P,
    var_gen: Gen,
    env: Environment<String, Variable>
}

impl<'pool, P: AsGenerationPool> Generator<'pool, P> {
    /// Constructs a new IR generator.
    pub fn new(ast: Vec<Handle<Stmt>>, pool: &'pool mut P) -> Self {
        Self {
            ast,
            pool,
            var_gen: Gen::new(),
            env: Environment::new()
        }
    }

    /// A new IR variable unique to this generator.
    fn new_var(&mut self) -> Variable {
        Variable::from(self.var_gen.next())
    }

    /// Generates IR for `expr` at the end of `seq`, returning its result.
    fn gen_expr(&mut self, expr: Handle<Expr>, seq: &mut Seq) -> Operand {
        match &expr.value {
            ExprValue::ConstantInt(value) => Operand::Constant(*value),
            ExprValue::InfixBop(lhs, op, rhs) => {
                let lhs = self.gen_expr(*lhs, seq);
                let rhs = self.gen_expr(*rhs, seq);
                let result = self.new_var();
                match op.ty {
                    TokenType::Plus => {
                        seq.push(Ir::Add(result, lhs, rhs), self.pool);
                    }
                    TokenType::Times => {
                        seq.push(Ir::Mul(result, lhs, rhs), self.pool);
                    }
                    _ => todo!("haven't implemented all infix bops")
                }
                Operand::Variable(result)
            }
            ExprValue::BoundName(name) => {
                Operand::Variable(*self.env.find(name.value.clone()).expect(
                    "unbound name should have been caught in type inference"
                ))
            }
            ExprValue::PostfixBop(array, op1, index, op2)
                if op1.ty == TokenType::LeftBracket
                    && op2.ty == TokenType::RightBracket =>
            {
                let array_operand = self.gen_expr(*array, seq);
                let index_operand = self.gen_expr(*index, seq);
                let result = self.new_var();
                seq.push(
                    Ir::Load {
                        result,
                        value: array_operand,
                        index: index_operand
                    },
                    self.pool
                );
                Operand::Variable(result)
            }
            ExprValue::ArrayLiteral(elements, _) => {
                let TypeValue::Array(element_type, element_count) =
                    self.pool.get_ty(expr).value
                else {
                    panic!();
                };
                let LiquidTypeValue::Equal(element_count) = element_count.value
                else {
                    panic!();
                };
                let element_size = element_type.size();

                let result = self.new_var();
                seq.push(
                    Ir::LocalAlloc(result, element_size, element_count),
                    self.pool
                );
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
                    f: Name::from_native(
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

    fn gen_stmt(&mut self, node: &Stmt, block: BasicBlockCell) {
        match &node.value {
            StmtValue::LetBinding {
                name,
                hint: _,
                value
            } => {
                let value_operand = self.gen_expr(value, block.clone());
                let name_var = Variable::new();
                self.env.bind(name.value.clone(), name_var);
                block.as_mut().add(Ir::Assign(name_var, value_operand));
            }
            StmtValue::Return {
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
        body: &Vec<Stmt>
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
            self.gen_stmt(node, entry.clone());
        }
        self.env.pop();
        let arg_tys = params.iter().map(|(_, ty)| ty.clone()).collect();
        let ret_ty = Box::new(ret);
        GeneratedTopLevel::Function {
            label: Label::from(
                Name::from_native(name.value.clone(), &arg_tys, &ret_ty),
                Visibility::Private
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
            StmtValue::Function {
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
