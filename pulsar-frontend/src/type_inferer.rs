//! Implements a standard Hindley-Milner type inference algorithm.
//!
//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::token::{Token, TokenType};
use crate::{
    ast::{
        expr::{Expr, ExprValue},
        node::NodeInterface,
        stmt::{Stmt, StmtValue},
        stmt_ty::StmtType,
        ty::{LiquidType, LiquidTypeValue, Type, TypeValue},
        AsASTPool
    },
    attribute::Attribute
};
use pulsar_utils::{
    disjoint_sets::DisjointSets,
    environment::Environment,
    error::{Error, ErrorBuilder, ErrorCode, ErrorManager, Level, Style},
    id::Gen,
    loc::Span,
    pool::{AsPool, Handle}
};
use std::{
    collections::{HashMap, HashSet},
    iter::zip,
    mem
};

struct UnificationConstraint<T> {
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

type TypeConstraint = UnificationConstraint<Type>;
type LiquidTypeConstraint = UnificationConstraint<LiquidType>;

pub trait AsInferencePool:
    AsASTPool + AsPool<TypeConstraint, ()> + AsPool<LiquidTypeConstraint, ()> {
}

pub struct TypeInferer<'pool, 'err, P: AsInferencePool> {
    ast: Vec<Handle<Stmt>>,
    env: Environment<String, Handle<Type>>,
    type_constraints: Vec<Handle<TypeConstraint>>,
    liquid_type_constraints: Vec<Handle<LiquidTypeConstraint>>,
    gen: Gen,
    pool: &'pool mut P,
    error_manager: &'err mut ErrorManager
}

impl<'pool, 'err, P: AsInferencePool> TypeInferer<'pool, 'err, P> {
    pub fn new(
        ast: Vec<Handle<Stmt>>, pool: &'pool mut P,
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
    pub fn infer(mut self) -> Option<Vec<Handle<Stmt>>> {
        let ast = mem::take(&mut self.ast);

        for top_level in &ast {
            self.register_top_level_bindings(*top_level);
        }

        for stmt in &ast {
            self.visit_stmt(*stmt)?;
        }

        let substitution = self.unify_constraints()?;
        for (ty, sub_ty) in &substitution {
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

    fn warn_dead_code(
        &mut self, func_name: &Token, dead_node: &Stmt, term_node: &Stmt
    ) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Warning)
                .with_code(ErrorCode::StaticAnalysisIssue)
                .span(dead_node)
                .message("Statement is never reached".into())
                .build()
        );
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Secondary)
                .at_level(Level::Warning)
                .span(term_node)
                .continues()
                .explain(format!(
                    "Returned from function `{}` here",
                    func_name.value
                ))
                .build()
        );
    }

    fn report_missing_return(&mut self, func_name: &Token) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::InvalidTopLevelConstruct)
                .span(func_name)
                .message(format!(
                    "Function `{}` does not return from all paths",
                    func_name.value
                ))
                .fix("Consider adding a `return` statement at the end of the function".into())
                .build()
        );
    }

    fn report_unbound_name(&mut self, name: &Token) {
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
                .span(&ty)
                .message(format!("Ambiguous type `{}`", ty))
                .explain(explain)
                .build()
        );
    }

    fn report_failed_purity_derivation(
        &mut self, pure_token: &Token, name: &Token, impure_node: &Stmt
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

    fn report_called_non_function(&mut self, name: &Token) {
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

    // fn report_invalid_operation(&mut self, explain: String, ctx: &Token) {
    //     self.report(
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
    fn register_top_level_bindings(&mut self, stmt: Handle<Stmt>) {
        if let StmtValue::Function {
            ref name,
            open_paren: _,
            ref params,
            ref close_paren,
            ret,
            body: _
        } = stmt.value.clone()
        {
            let args = params
                .iter()
                .map(|(_, arg_type)| self.pool.duplicate(*arg_type))
                .collect();
            let ret_type = ret;
            let func_type = self.pool.new(
                TypeValue::Function { args, ret },
                name.clone(),
                if ret_type.has_attribute(Attribute::Generated) {
                    close_paren
                } else {
                    ret_type.end_token()
                }
                .clone()
            );
            self.bind_top_level(&name.value, func_type);
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
            source.as_ref().start_token().clone(),
            source.as_ref().end_token().clone()
        )
    }

    /// Extracts constraints from the expression `expr` and returns either a
    /// type variable or fully resolved type for it.
    fn visit_expr(&mut self, expr: Handle<Expr>) -> Option<Handle<Type>> {
        let expr_type = match &expr.value {
            ExprValue::ConstantInt(_) => self.pool.generate(
                TypeValue::Int64,
                expr.start_token().clone(),
                expr.end_token().clone()
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
                        expr.start_token().clone(),
                        expr.end_token().clone()
                    );
                    self.pool.generate(
                        TypeValue::Array(first_type, liquid_type),
                        expr.start_token().clone(),
                        expr.end_token().clone()
                    )
                }
            }
            // for now, all ops are on integers
            ExprValue::PrefixOp(op, rhs) => {
                let op_rhs_type = self.pool.generate(
                    TypeValue::Int64,
                    op.clone(),
                    op.clone()
                );
                let op_result_type = self.pool.duplicate(op_rhs_type);
                let rhs_type = self.visit_expr(*rhs)?;
                self.new_constraint(op_rhs_type, rhs_type);
                op_result_type
            }
            // for now, all ops are on integers
            ExprValue::InfixBop(lhs, op, rhs)
            | ExprValue::PostfixBop(lhs, op, rhs, _) => {
                let op_lhs_type = self.pool.generate(
                    TypeValue::Int64,
                    op.clone(),
                    op.clone()
                );
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

    fn visit_stmt(&mut self, stmt: Handle<Stmt>) -> Option<StmtType> {
        let stmt_type = match &stmt.value {
            StmtValue::Function {
                name,
                params,
                open_paren: _,
                ret,
                close_paren: _,
                body
            } => {
                let mut return_type = StmtType::Nonterminal;
                let mut return_stmt = None;

                self.env.push();
                for (param_name, param_type) in params {
                    self.env.bind(param_name.value.clone(), *param_type);
                }
                for stmt in body {
                    if let Some(return_stmt) = return_stmt {
                        self.warn_dead_code(name, &stmt, return_stmt)
                    }
                    if let StmtType::Terminal(stmt_return_type) =
                        self.visit_stmt(*stmt)?
                    {
                        self.new_constraint(*ret, stmt_return_type);
                        return_type = StmtType::Terminal(stmt_return_type);
                        return_stmt = Some(stmt);
                    }
                }
                self.env.pop();

                if matches!(return_type, StmtType::Nonterminal) {
                    self.report_missing_return(name);
                    return None;
                }
                return_type
            }
            StmtValue::LetBinding { name, hint, value } => {
                let value_type = self.visit_expr(*value)?;
                if let Some(hint) = hint {
                    self.new_constraint(*hint, value_type);
                }
                self.env.bind(name.value.clone(), value_type);
                StmtType::Nonterminal
            }
            StmtValue::Return { ret_token, value } => {
                StmtType::Terminal(if let Some(value) = value {
                    self.visit_expr(*value)?
                } else {
                    self.pool.generate(
                        TypeValue::Unit,
                        ret_token.clone(),
                        ret_token.clone()
                    )
                })
            }
        };
        self.pool.set_ty(stmt, stmt_type.clone());
        Some(stmt_type)
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
                (expected_value, actual_value) => {
                    if mem::discriminant(expected_value)
                        == mem::discriminant(actual_value)
                    {
                        for (expected_subterm, actual_subterm) in
                            zip(expected_rep.subterms(), actual_rep.subterms())
                        {
                            self.derive_constraint(
                                expected_subterm,
                                actual_subterm,
                                constraint
                            );
                        }
                    } else {
                        self.report_unification_failure(constraint, None);
                        return Err(());
                    }
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
