// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

use std::path::Iter;

use super::{
    label::{Label, Name, Visibility},
    operand::Operand,
    variable::Variable,
    Ir
};
use crate::{
    component::Component,
    control::{AsControlPool, Control, SeqParBuilder},
    memory::Memory
};
use pulsar_frontend::{
    ast::{
        expr::{Expr, ExprValue},
        pretty_print::PrettyPrint,
        stmt::{Param, Stmt, StmtValue},
        ty::Type,
        AsASTPool
    },
    token::{Token, TokenType}
};
use pulsar_utils::{environment::Environment, id::Gen, pool::Handle};

pub trait AsGenerationPool: AsASTPool + AsControlPool {}

/// IR generator.
pub struct Generator<'pool, P: AsGenerationPool> {
    ast: Vec<Handle<Stmt>>,
    index: usize,
    pool: &'pool mut P,
    var_gen: Gen,
    env: Environment<String, Variable>,
    memories: Environment<String, Memory>
}

impl<'pool, P: AsGenerationPool> Generator<'pool, P> {
    /// Constructs a new IR generator.
    pub fn new(ast: Vec<Handle<Stmt>>, pool: &'pool mut P) -> Self {
        Self {
            ast,
            index: 0,
            pool,
            var_gen: Gen::new(),
            env: Environment::new(),
            memories: Environment::new()
        }
    }

    /// A new IR variable unique to this generator.
    fn new_var(&mut self) -> Variable {
        Variable::from(self.var_gen.next())
    }

    /// Generates IR for `expr` in `par`, returning its result.
    fn gen_expr(
        &mut self, expr: Handle<Expr>, control: &mut SeqParBuilder<P>
    ) -> Operand {
        match &expr.value {
            ExprValue::ConstantInt(value) => Operand::Constant(*value),
            ExprValue::InfixBop(lhs, op, rhs) => {
                let lhs = self.gen_expr(*lhs, control);
                let rhs = self.gen_expr(*rhs, control);
                let result = self.new_var();
                match op.ty {
                    TokenType::Plus => {
                        control.push(Ir::Add(result, lhs, rhs));
                    }
                    TokenType::Times => {
                        control.push(Ir::Mul(result, lhs, rhs));
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
                let array_operand = self.gen_expr(*array, control);
                let index_operand = self.gen_expr(*index, control);
                let result = self.new_var();
                control.push(Ir::Load {
                    result,
                    value: array_operand,
                    index: index_operand
                });
                Operand::Variable(result)
            }
            _ => todo!()
        }
    }

    fn gen_stmt(&mut self, stmt: Handle<Stmt>, control: &mut SeqParBuilder<P>) {
        match &stmt.value {
            StmtValue::Let {
                name,
                hint: _,
                value
            } => {
                let value_operand = self.gen_expr(*value, control);
                let name_var = self.new_var();
                self.env.bind(name.value.clone(), name_var);
                control.push(Ir::Assign(name_var, value_operand));
            }
            StmtValue::Return {
                ret_token: _,
                value
            } => {
                todo!();
                // let value_operand = value
                //     .as_ref()
                //     .map(|value| self.gen_expr(value.as_ref(), par));
                // block.as_mut().add(Ir::Return(value_operand));
            }
            StmtValue::Divider(_) => {
                control.split();
            }
            _ => {}
        }
    }

    pub fn gen_comp(&mut self, name: &Token) -> Component {}
}

impl<'pool, P: AsGenerationPool> Iterator for Generator<'pool, P> {
    type Item = Component;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.ast.len() {
            return None;
        }
        let func = self.ast[self.index];
        self.index += 1;
        match &func.value {
            StmtValue::Function {
                name,
                inputs: params,
                open_paren,
                outputs: ret,
                close_paren,
                body
            } => Some(self.gen_comp()),
            _ => panic!()
        }
    }
}
