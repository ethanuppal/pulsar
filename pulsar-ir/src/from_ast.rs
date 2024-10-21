//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::{
    label::{Label, Name, Visibility},
    port::Port,
    variable::Variable
};
use crate::{
    cell::Cell,
    component::Component,
    control::{AsControlPool, Control, ControlBuilder, For},
    pass::PassRunner
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

pub trait AsGeneratorPool:
    AsControlPool + AsPool<Port, ()> + AsPool<Cell, ()> {
}

/// Every assignment statement corresponds to a single IR assignment to the
/// appropriate lvalue.
pub fn ast_to_ir<P: AsGeneratorPool>(
    ast: AST, mut pass_runner: PassRunner<P>, pool: &mut P, var_gen: &mut Gen
) -> Vec<Component> {
    ast.into_iter()
        .map(|decl| match &decl.value {
            DeclValue::Function {
                func: _,
                name,
                inputs,
                outputs,
                body
            } => ComponentGenerator::new(var_gen).gen(
                *name,
                inputs,
                outputs,
                body,
                &mut pass_runner,
                pool
            )
        })
        .collect()
}

pub struct ComponentGenerator<'gen> {
    var_gen: &'gen mut Gen,
    env: Environment<String, Variable> // cells: Environment<Variable, Cell>
}

impl<'gen> ComponentGenerator<'gen> {
    pub fn new(var_gen: &'gen mut Gen) -> Self {
        Self {
            var_gen,
            env: Environment::new()
        }
    }

    pub fn new_var(&mut self) -> Variable {
        Variable::from(self.var_gen.next())
    }

    pub fn gen<P: AsGeneratorPool>(
        mut self, name: Handle<Token>, inputs: &[Param], outputs: &[Param],
        body: &[Handle<Stmt>], pass_runner: &mut PassRunner<P>, pool: &mut P
    ) -> Component {
        let mut input_cells = Vec::new();
        let mut output_cells = Vec::new();
        for (params, cells) in
            [(inputs, &mut input_cells), (outputs, &mut output_cells)]
        {
            for (port_name, port_type) in params {
                let var = self.new_var();
                self.env.bind(port_name.value.clone(), var);
                cells.push((var, pool.add(Cell::from(port_type))))
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

        let mut builder = ControlBuilder::new(pool);
        for stmt in body {
            self.gen_stmt(*stmt, &mut builder)
        }
        let cfg = builder.take();

        let mut comp =
            Component::new(label, input_cells, output_cells, pool.add(cfg));
        pass_runner.run(&mut comp, pool);
        comp
    }

    /// Generates IR for `expr` in `par`, returning its result. If `expr` is an
    /// lvalue, no additional control is built.
    fn gen_expr<P: AsGeneratorPool>(
        &mut self, expr: Handle<Expr>, builder: &mut ControlBuilder<P>
    ) -> Port {
        match &expr.value {
            ExprValue::ConstantInt(value) => Port::Constant(*value),
            ExprValue::InfixBop(lhs, op, rhs) => {
                let lhs = self.gen_expr(*lhs, builder);
                let rhs = self.gen_expr(*rhs, builder);
                let result = self.new_var();
                match op.ty {
                    TokenType::Plus => {
                        builder.push_add(result, lhs, rhs);
                    }
                    TokenType::Times => {
                        builder.push_mul(result, lhs, rhs);
                    }
                    _ => todo!("haven't implemented all infix bops")
                }
                Port::Variable(result)
            }
            ExprValue::BoundName(name) => {
                Port::Variable(*self.env.find(name.value.clone()).unwrap_or_else(|| panic!("unbound name `{}` should have been caught in type inference", name.value)))
            }
            ExprValue::PostfixBop(array, op1, index, op2)
                if op1.ty == TokenType::LeftBracket
                    && op2.ty == TokenType::RightBracket =>
            {
                let array_port = self.gen_expr(*array, builder);
                let index_port = self.gen_expr(*index, builder);
                Port::PartialAccess(builder.add_port(array_port), builder.add_port(index_port))
            },
            ExprValue::ArrayLiteral(elements, _should_continue) => {
                let result = Port::Variable(self.new_var());
                let result_handle = builder.add_port(result.clone()); // leakedA
                let mut i = 0;
                #[allow(clippy::explicit_counter_loop)] // for 2nd loop of zeros
                for element in elements {
                    let index = builder.new_const(i as i64);
                    let element_port = self.gen_expr(*element, builder);
                    builder.push_assign(Port::PartialAccess(result_handle, index), element_port);
                    i += 1
                }
                // TODO: figure out a nice way to get the liquid type for array length here
                result
            },
            _ => todo!()
        }
    }

    fn gen_stmt<P: AsGeneratorPool>(
        &mut self, stmt: Handle<Stmt>, builder: &mut ControlBuilder<P>
    ) {
        match &stmt.value {
            StmtValue::Let {
                name,
                hint: _,
                value
            } => {
                let value_port = self.gen_expr(*value, builder);
                let name_var = self.new_var();
                self.env.bind(name.value.clone(), name_var);
                builder.push_assign(name_var, value_port);
            }
            StmtValue::Assign(lhs, _, rhs) => {
                let rhs_port = self.gen_expr(*rhs, builder);
                let lhs_port = self.gen_expr(*lhs, builder);
                builder.push_assign(lhs_port, rhs_port);
            }
            StmtValue::Divider(_) => {
                builder.split();
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
                let lower_port = self.gen_expr(*lower, builder);
                let upper_port = self.gen_expr(*exclusive_upper, builder);
                let body = builder.with_pool(|pool| {
                    let mut builder = ControlBuilder::new(pool);
                    self.env.push();
                    for stmt in body {
                        self.gen_stmt(*stmt, &mut builder);
                    }
                    self.env.pop();
                    let control = builder.take();
                    pool.add(control)
                });
                self.env.pop();
                builder.push(Control::For(For::new(
                    variant, lower_port, upper_port, None, body
                )));
            }
        }
    }
}
