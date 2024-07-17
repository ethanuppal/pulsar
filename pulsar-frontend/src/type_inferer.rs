//! Implements a standard Hindley-Milner type inference algorithm.
//!
//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::token::Token;
use crate::ast::{
    decl::{Decl, DeclValue},
    expr::{Expr, ExprValue},
    node::NodeInterface,
    stmt::{Stmt, StmtValue},
    ty::{LiquidType, LiquidTypeValue, Type, TypeValue},
    AsASTPool
};
use pulsar_utils::{
    disjoint_sets::DisjointSets,
    environment::Environment,
    error::{Error, ErrorBuilder, ErrorCode, ErrorManager, Level, Style},
    id::Gen,
    pool::{AsPool, Handle, HandleArray}
};
use std::iter::zip;

pub struct UnificationConstraint<T> {
    expected: Handle<T>,
    actual: Handle<T>,
    source: Option<Handle<Self>>
}

impl<T> UnificationConstraint<T> {
    /// Constructs a constraint that sets `expected` equal to `actual`.
    pub fn new(expected: Handle<T>, actual: Handle<T>) -> Self {
        Self {
            expected,
            actual,
            source: None
        }
    }

    pub fn derived(
        expected: Handle<T>, actual: Handle<T>, source: Handle<Self>
    ) -> Self {
        Self {
            expected,
            actual,
            source: Some(source)
        }
    }

    pub fn expected(&self) -> Handle<T> {
        self.source
            .map_or(self.expected, |source| source.expected())
    }

    pub fn actual(&self) -> Handle<T> {
        self.source.map_or(self.actual, |source| source.actual())
    }
}

pub type TypeConstraint = UnificationConstraint<Type>;
pub type LiquidTypeConstraint = UnificationConstraint<LiquidType>;

pub trait AsInferencePool:
    AsASTPool + AsPool<TypeConstraint, ()> + AsPool<LiquidTypeConstraint, ()> {
}

pub struct TypeInferer<'pool, 'err, P: AsInferencePool> {
    ast: HandleArray<Decl>,
    env: Environment<String, Handle<Type>>,
    type_constraints: Vec<Handle<TypeConstraint>>,
    liquid_type_constraints: Vec<Handle<LiquidTypeConstraint>>,
    gen: Gen,
    pool: &'pool mut P,
    error_manager: &'err mut ErrorManager
}

impl<'pool, 'err, P: AsInferencePool> TypeInferer<'pool, 'err, P> {
    pub fn new(
        ast: HandleArray<Decl>, pool: &'pool mut P,
        error_manager: &'err mut ErrorManager
    ) -> Self {
        Self {
            ast,
            env: Environment::new(),
            type_constraints: Vec::new(),
            liquid_type_constraints: Vec::new(),
            gen: Gen::new(),
            pool,
            error_manager
        }
    }

    /// Establishes a top-level binding for the type `ty` of `name`, useful for
    /// allowing functions to call other functions or external/FFI declarations.
    pub fn bind_top_level<S: AsRef<str>>(&mut self, name: S, ty: Handle<Type>) {
        self.env.bind_base(name.as_ref().to_string(), ty);
    }

    /// Performs control-flow analysis on functions and infers the types of
    /// nodes and expression in the program `program`, returning the
    /// annotated AST if no error occured.
    pub fn infer(mut self) -> Option<HandleArray<Decl>> {
        for decl in self.ast {
            self.register_top_level_bindings(decl);
        }

        for decl in self.ast {
            self.visit_decl(decl)?;
        }

        let substitution = self.unify_constraints()?;
        for (ty, sub_ty) in &substitution {
            #[allow(clippy::single_match)] // since might add more later
            match sub_ty.value {
                TypeValue::Var(_) => {
                    self.report_ambiguous_type(
                        *sub_ty,
                        "Type variable not resolved (bug?)".into()
                    );
                    return None;
                }
                _ => {}
            }
            let mut ty = *ty;
            ty.value = sub_ty.value.clone();
        }

        Some(self.ast)
    }

    fn report(&mut self, error: Error) {
        self.error_manager.record(error);
    }

    fn report_unbound_name(&mut self, name: Handle<Token>) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::UnboundName)
                .span(name)
                .message(format!(
                    "Unbound function or variable `{}`",
                    name.value
                ))
                .build()
        );
    }

    fn report_ambiguous_type(
        &mut self, ty: Handle<Type>, /* expr: &Expr, */ explain: String
    ) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::StaticAnalysisIssue)
                .span(ty)
                .message(format!("Ambiguous type `{}`", ty))
                .explain(explain)
                .build()
        );
    }

    fn report_failed_purity_derivation(
        &mut self, pure_token: Handle<Token>, name: Handle<Token>,
        impure_node: &Stmt
    ) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::StaticAnalysisIssue)
                .span(impure_node)
                .message(format!(
                    "Impure statement in `pure` function `{}`",
                    name.value
                ))
                .build()
        );
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Secondary)
                .at_level(Level::Error)
                .with_code(ErrorCode::StaticAnalysisIssue)
                .span(pure_token)
                .continues()
                .explain("Function declared pure here".into())
                .fix("Consider marking called functions with `pure`".into())
                .build()
        );
    }

    fn report_called_non_function(&mut self, name: Handle<Token>) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::StaticAnalysisIssue)
                .span(name)
                .message(format!(
                    "Cannot call non-function value `{}`",
                    name.value
                ))
                .build()
        );
    }

    // fn report_invalid_operation(&mut self, explain: String, ctx:
    // Handle<Token>) {     self.report(
    //         ErrorBuilder::new()
    //             .of_style(Style::Primary)
    //             .at_level(Level::Error)
    //             .with_code(ErrorCode::StaticAnalysisIssue)
    //             .at_region(ctx)
    //             .message("Invalid operation".into())
    //             .explain(explain)
    //             .build()
    //     );
    // }

    fn report_unification_failure(
        &mut self, constraint: Handle<TypeConstraint>, fix: Option<String>
    ) {
        let expected = constraint.expected();
        let actual = constraint.actual();
        let mut builder = ErrorBuilder::new()
            .of_style(Style::Primary)
            .at_level(Level::Error)
            .with_code(ErrorCode::StaticAnalysisIssue)
            .span(expected)
            .message(format!(
                "Failed to unify types `{}` and `{}`",
                expected, actual
            ))
            .explain(format!("Expected `{}` here", expected));
        if let Some(fix) = fix {
            builder = builder.fix(fix);
        }
        self.report(builder.build());
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Secondary)
                .at_level(Level::Error)
                .with_code(ErrorCode::StaticAnalysisIssue)
                .span(actual)
                .continues()
                .explain(format!("but received `{}`.", actual))
                .build()
        );
    }
}

impl<'pool, 'err, P: AsInferencePool> TypeInferer<'pool, 'err, P> {
    fn register_top_level_bindings(&mut self, stmt: Handle<Decl>) {
        match &stmt.value {
            DeclValue::Function {
                func: _,
                name,
                inputs,
                outputs,
                body: _
            } => {
                let inputs = inputs
                    .iter()
                    .map(|(_, arg_type)| self.pool.duplicate(*arg_type))
                    .collect();
                let outputs = outputs
                    .iter()
                    .map(|(_, arg_type)| self.pool.duplicate(*arg_type))
                    .collect();
                let func_type = self.pool.new(
                    TypeValue::Function { inputs, outputs },
                    *name,
                    *name
                );
                self.bind_top_level(&name.value, func_type);
            }
        }
    }

    fn new_constraint(&mut self, expected: Handle<Type>, actual: Handle<Type>) {
        let handle = self.pool.add(TypeConstraint::new(expected, actual));
        self.type_constraints.push(handle);
    }

    fn derive_constraint(
        &mut self, expected: Handle<Type>, actual: Handle<Type>,
        source: Handle<TypeConstraint>
    ) {
        let handle = self
            .pool
            .add(TypeConstraint::derived(expected, actual, source));
        self.type_constraints.push(handle);
    }

    fn new_type_var<N: NodeInterface, NRef: AsRef<N>>(
        &mut self, source: NRef
    ) -> Handle<Type> {
        self.pool.generate(
            TypeValue::Var(self.gen.next()),
            source.as_ref().start_token(),
            source.as_ref().end_token()
        )
    }

    /// Extracts constraints from the expression `expr` and returns either a
    /// type variable or fully resolved type for it.
    fn visit_expr(&mut self, expr: Handle<Expr>) -> Option<Handle<Type>> {
        let expr_type = match &expr.value {
            ExprValue::ConstantInt(_) => self.pool.generate(
                TypeValue::Int64,
                expr.start_token(),
                expr.end_token()
            ),
            ExprValue::BoundName(_) => self.new_type_var(expr),
            ExprValue::MemberAccess(_, _) => todo!(),
            ExprValue::Call(_, _) => todo!(),
            ExprValue::ArrayLiteral(elements, should_continue) => {
                if elements.is_empty() && should_continue.is_some() {
                    self.new_type_var(expr)
                } else {
                    let element_count = elements.len();
                    let mut elements = elements.iter();
                    let first = elements.next().unwrap();
                    let first_type = self.visit_expr(*first)?;
                    for other in elements {
                        let other_type = self.visit_expr(*other)?;
                        self.new_constraint(first_type, other_type);
                    }
                    let liquid_type = self.pool.generate(
                        if should_continue.is_some() {
                            LiquidTypeValue::All
                        } else {
                            LiquidTypeValue::Equal(element_count)
                        },
                        expr.start_token(),
                        expr.end_token()
                    );
                    self.pool.generate(
                        TypeValue::Array(first_type, liquid_type),
                        expr.start_token(),
                        expr.end_token()
                    )
                }
            }
            // for now, all ops are on integers
            ExprValue::PrefixOp(op, rhs) => {
                let op_rhs_type =
                    self.pool.generate(TypeValue::Int64, *op, *op);
                let op_result_type = self.pool.duplicate(op_rhs_type);
                let rhs_type = self.visit_expr(*rhs)?;
                self.new_constraint(op_rhs_type, rhs_type);
                op_result_type
            }
            // for now, all ops are on integers
            ExprValue::InfixBop(lhs, op, rhs)
            | ExprValue::PostfixBop(lhs, op, rhs, _) => {
                let op_lhs_type =
                    self.pool.generate(TypeValue::Int64, *op, *op);
                let op_rhs_type = self.pool.duplicate(op_lhs_type);
                let op_result_type = self.pool.duplicate(op_rhs_type);
                let lhs_type = self.visit_expr(*lhs)?;
                let rhs_type = self.visit_expr(*rhs)?;
                self.new_constraint(op_lhs_type, lhs_type);
                self.new_constraint(op_rhs_type, rhs_type);
                op_result_type
            }
        };
        self.pool.set_ty(expr, expr_type);
        Some(expr_type)
    }

    fn visit_stmt(&mut self, stmt: Handle<Stmt>) -> Option<()> {
        match &stmt.value {
            StmtValue::Let { name, hint, value } => {
                let value_type = self.visit_expr(*value)?;
                if let Some(hint) = hint {
                    self.new_constraint(*hint, value_type);
                }
                self.env.bind(name.value.clone(), value_type);
            }
            StmtValue::Assign(lhs, equals, rhs) => {
                // let lhs_type =
            }
            StmtValue::Divider(_) => todo!()
        }
        Some(())
    }

    fn visit_decl(&mut self, decl: Handle<Decl>) -> Option<()> {
        match &decl.value {
            DeclValue::Function {
                func: _,
                name: _,
                ref inputs,
                ref outputs,
                ref body
            } => {
                self.env.push();
                for (param_name, param_type) in inputs {
                    self.env.bind(param_name.value.clone(), *param_type);
                }
                for (param_name, param_type) in outputs {
                    self.env.bind(param_name.value.clone(), *param_type);
                }

                {
                    self.env.push();
                    for stmt in body {
                        self.visit_stmt(*stmt)?;
                    }
                    self.env.pop();
                }

                self.env.pop();
            }
        }
        Some(())
    }

    /// Whenever possible, pass `lhs` as the type to be unified into `rhs`.
    fn unify(
        &mut self, dsu: &mut DisjointSets<Handle<Type>>,
        constraint: Handle<TypeConstraint>
    ) -> Result<(), ()> {
        let expected = constraint.expected;
        let expected_rep =
            dsu.find(expected).expect("expected type not added to dsu");
        let actual = constraint.actual;
        let actual_rep =
            dsu.find(actual).expect("actual type not added to dsu");
        if expected_rep != actual_rep {
            match (&expected_rep.value, &actual_rep.value) {
                (TypeValue::Var(_), TypeValue::Var(_)) => {
                    dsu.union(actual, expected, false)
                        .expect("failed to union");
                }
                (TypeValue::Var(_), _) => {
                    dsu.union(actual, expected, false)
                        .expect("failed to union");
                }
                (_, TypeValue::Var(_)) => {
                    dsu.union(expected, actual, false)
                        .expect("failed to union");
                }
                _ if expected.can_unify_with(&actual) => {
                    for (expected_subterm, actual_subterm) in
                        zip(expected_rep.subterms(), actual_rep.subterms())
                    {
                        self.derive_constraint(
                            expected_subterm,
                            actual_subterm,
                            constraint
                        );
                    }
                }
                _ => {
                    self.report_unification_failure(constraint, None);
                    return Err(());
                }
            }
        }
        Ok(())
    }

    fn unify_constraints(&mut self) -> Option<DisjointSets<Handle<Type>>> {
        let mut dsu = DisjointSets::new();
        while let Some(constraint) = self.type_constraints.pop() {
            self.unify(&mut dsu, constraint).ok()?;
        }
        dsu.collapse();
        Some(dsu)
    }
}
