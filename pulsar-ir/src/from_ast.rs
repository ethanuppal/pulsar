//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::{
    label::{Label, Name, Visibility},
    operand::Operand,
    variable::Variable,
    Ir
};
use crate::{
    cell::Cell,
    component::Component,
    control::{AsControlPool, Control, For, SeqParBuilder},
    memory::Memory
};
use pulsar_frontend::{
    ast::{
        decl::{DeclValue, Param},
        expr::{Expr, ExprValue},
        stmt::{Stmt, StmtValue},
        AST
    },
    token::{Token, TokenType}
};
use pulsar_utils::{
    environment::Environment,
    id::Gen,
    pool::{AsPool, Handle}
};

pub trait AsGeneratorPool: AsControlPool + AsPool<Cell, ()> {}

#[derive(Default)]
pub struct ComponentGenerator {
    var_gen: Gen,
    env: Environment<String, Variable>,
    cells: Environment<Variable, Cell>
}

pub fn ast_to_ir<P: AsGeneratorPool>(ast: AST, pool: &mut P) -> Vec<Component> {
    ast.into_iter()
        .map(|decl| match &decl.value {
            DeclValue::Function {
                func: _,
                name,
                inputs,
                outputs,
                body
            } => ComponentGenerator::new()
                .gen(pool, *name, inputs, outputs, body),
            _ => panic!("tried to turn not-a-function into a component")
        })
        .collect()
}

impl ComponentGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn gen<P: AsGeneratorPool>(
        mut self, pool: &mut P, name: Handle<Token>, inputs: &[Param],
        outputs: &[Param], body: &[Handle<Stmt>]
    ) -> Component {
        let mut input_cells = Vec::new();
        let mut output_cells = Vec::new();
        for (params, cells) in
            [(inputs, &mut input_cells), (outputs, &mut output_cells)]
        {
            for (port_name, port_type) in params {
                let var = self.new_var();
                self.env.bind(port_name.value.clone(), var);
                cells.push(pool.add(Cell::from(port_type)))
            }
        }

        let input_types = inputs
            .iter()
            .map(|(_, input_type)| *input_type)
            .collect::<Vec<_>>();
        let output_types = outputs
            .iter()
            .map(|(_, output_type)| *output_type)
            .collect::<Vec<_>>();

        let label = Label::from(
            Name::from_native(&name.value, &input_types, &output_types),
            Visibility::Public
        );

        let mut control = SeqParBuilder::new(pool);
        for stmt in body {
            self.gen_stmt(*stmt, &mut control)
        }

        let mut comp =
            Component::new(label, input_cells, output_cells, control.into());

        comp
    }

    /// A new IR variable unique to this generator.
    fn new_var(&mut self) -> Variable {
        Variable::from(self.var_gen.next())
    }

    /// Generates IR for `expr` in `par`, returning its result.
    fn gen_expr<P: AsControlPool>(
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
                Operand::Variable(*self.env.find(name.value.clone()).unwrap_or_else(|| panic!("unbound name `{}` should have been caught in type inference", name.value)))
            }
            ExprValue::PostfixBop(array, op1, index, op2)
                if op1.ty == TokenType::LeftBracket
                    && op2.ty == TokenType::RightBracket =>
            {
                let array_operand = self.gen_expr(*array, control);
                let index_operand = self.gen_expr(*index, control);
                let result = self.new_var();
                control.push(Ir::Assign(result, Operand::PartialAccess(Box::new(array_operand), Box::new(index_operand))));
                Operand::Variable(result)
            }
            _ => todo!()
        }
    }

    fn gen_stmt<P: AsControlPool>(
        &mut self, stmt: Handle<Stmt>, control: &mut SeqParBuilder<P>
    ) {
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
            StmtValue::Assign(lhs, _, rhs) => {
                let Operand::Variable(lhs_var) = self.gen_expr(*lhs, control)
                else {
                    panic!("rvalue in lhs of expr not caught already");
                };
                let rhs_operand = self.gen_expr(*rhs, control);
                control.push(Ir::Assign(lhs_var, rhs_operand));
            }
            StmtValue::Divider(_) => {
                control.split();
            }
            StmtValue::For {
                var,
                lower,
                exclusive_upper,
                body
            } => {
                self.env.push();
                let variant = self.new_var();
                self.env.bind(var.value.clone(), variant);
                let lower_operand = self.gen_expr(*lower, control);
                let upper_operand = self.gen_expr(*exclusive_upper, control);
                let body = control.with_pool(|pool| {
                    let mut builder = SeqParBuilder::new(pool);
                    self.env.push();
                    for stmt in body {
                        self.gen_stmt(*stmt, &mut builder);
                    }
                    self.env.pop();
                    let control = builder.into();
                    pool.add(control)
                });
                self.env.pop();
                control.push(Control::For(For::new(
                    variant,
                    lower_operand,
                    upper_operand,
                    body
                )));
            }
        }
    }
}
