//! Implements a standard Hindley-Milner type inference algorithm.
//!
//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::token::Token;
use crate::{
    ast::{
        decl::{Decl, DeclValue},
        expr::{Expr, ExprValue},
        node::NodeInterface,
        stmt::{Stmt, StmtValue},
        ty::{LiquidType, LiquidTypeValue, Type, TypeValue},
        AsASTPool
    },
    token::TokenType
};
use pulsar_utils::{
    disjoint_sets::DisjointSets,
    environment::Environment,
    error::{Error, ErrorBuilder, ErrorCode, ErrorManager, Level, Style},
    id::Gen,
    pool::{AsPool, Handle, HandleArray},
    span::SpanProvider
};
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    hash::Hash,
    iter::zip
};

#[derive(Debug)]
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

    pub fn immediate(&self) -> Option<&Self> {
        if self.source.is_some() {
            Some(self)
        } else {
            None
        }
    }
}

pub type TypeConstraint = UnificationConstraint<Type>;
pub type LiquidTypeConstraint = UnificationConstraint<LiquidType>;

trait HasTypeVars<T> {
    fn type_vars(&self) -> Vec<(Handle<T>, Option<String>)>;
}

impl HasTypeVars<LiquidType> for Handle<LiquidType> {
    fn type_vars(&self) -> Vec<(Handle<LiquidType>, Option<String>)> {
        vec![]
    }
}

impl HasTypeVars<Type> for Handle<Type> {
    fn type_vars(&self) -> Vec<(Handle<Type>, Option<String>)> {
        if let TypeValue::Var(_, name) = &self.value {
            vec![(*self, name.clone())]
        } else {
            self.subterms()
                .iter()
                .map(|subterm| subterm.type_vars())
                .collect::<Vec<_>>()
                .concat()
        }
    }
}

trait Unifier<
    T: NodeInterface + Eq + Hash + Display,
    P: AsPool<UnificationConstraint<T>, ()>
> {
    fn type_pool_mut(&mut self) -> &mut P;
    fn constraints(
        &mut self
    ) -> &mut VecDeque<Handle<UnificationConstraint<T>>>;

    fn new_constraint(&mut self, expected: Handle<T>, actual: Handle<T>) {
        let handle = self
            .type_pool_mut()
            .add(UnificationConstraint::new(expected, actual));

        self.constraints().push_back(handle);
        log::trace!("encountered constraint: {} = {}", expected, actual);
    }

    fn derive_constraint(
        &mut self, expected: Handle<T>, actual: Handle<T>,
        source: Handle<UnificationConstraint<T>>
    ) {
        let handle = self
            .type_pool_mut()
            .add(UnificationConstraint::derived(expected, actual, source));
        self.constraints().push_back(handle);

        log::trace!(
            "encountered constraint: {} = {} (from {} = {})",
            expected,
            actual,
            source.expected(),
            source.actual()
        );
    }

    fn unify_constraint(
        &mut self, dsu: &mut DisjointSets<Handle<T>>,
        constraint: Handle<UnificationConstraint<T>>
    ) -> Result<(), ()>;

    fn unify_constraints(&mut self) -> Option<DisjointSets<Handle<T>>> {
        let mut dsu = DisjointSets::new();
        while let Some(constraint) = self.constraints().pop_front() {
            dsu.add(constraint.expected);
            dsu.add(constraint.actual);
            self.unify_constraint(&mut dsu, constraint).ok()?;
        }
        dsu.collapse();
        Some(dsu)
    }
}

pub trait AsInferencePool:
    AsASTPool + AsPool<TypeConstraint, ()> + AsPool<LiquidTypeConstraint, ()> {
}

pub struct TypeInferer<'pool, 'err, P: AsInferencePool> {
    ast: HandleArray<Decl>,
    env: Environment<String, Handle<Type>>,
    type_constraints: VecDeque<Handle<TypeConstraint>>,
    liquid_type_constraints: VecDeque<Handle<LiquidTypeConstraint>>,
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
            type_constraints: VecDeque::new(),
            liquid_type_constraints: VecDeque::new(),
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

        {
            let substitution =
                Unifier::<Type, P>::unify_constraints(&mut self)?;
            for (ty, sub_ty) in &substitution {
                #[allow(clippy::single_match)] // since might add more later
                match sub_ty.value {
                    TypeValue::Var(..) => {
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
        }

        // TODO: remove this code dup
        {
            let substitution =
                Unifier::<LiquidType, P>::unify_constraints(&mut self)?;
            for (ty, sub_ty) in &substitution {
                #[allow(clippy::single_match)] // since might add more later
                match sub_ty.value {
                    LiquidTypeValue::All(..) => {
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

    fn report_ambiguous_type<T: NodeInterface + Display>(
        &mut self, ty: Handle<T>, /* expr: &Expr, */ explain: String
    ) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::AmbiguousType)
                .span(ty)
                .message(format!("Ambiguous type `{}`", ty))
                .explain(explain)
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

    fn report_unification_failure<
        T: SpanProvider + Display + Eq + Hash + Clone
    >(
        &mut self, constraint: Handle<UnificationConstraint<T>>,
        dsu: &mut DisjointSets<Handle<T>>, fix: Option<String>
    ) where
        Handle<T>: HasTypeVars<T> {
        let expected = dsu.find(constraint.expected()).unwrap();
        let actual = dsu.find(constraint.actual()).unwrap();

        let mut builder = ErrorBuilder::new()
            .of_style(Style::Primary)
            .at_level(Level::Error)
            .with_code(ErrorCode::UnificationFailure)
            .span(expected)
            .message(format!(
                "Failed to unify types `{}` and `{}`",
                expected, actual
            ))
            .explain(format!("Expected `{}` here,", expected));
        if let Some(fix) = fix {
            builder = builder.fix(fix);
        }
        self.report(builder.build());

        self.report(
            ErrorBuilder::new()
                .of_style(Style::Secondary)
                .at_level(Level::Error)
                .span(actual)
                .continues()
                .explain(format!("but received `{}` here.", actual))
                .build()
        );

        let expected_type_vars = expected.type_vars();
        let actual_type_vars = actual.type_vars();
        let type_vars = expected_type_vars
            .iter()
            .chain(&actual_type_vars)
            .flat_map(|(ty, name)| name.as_ref().map(|name| (ty, name)))
            .collect::<Vec<_>>();
        let mut seen = HashSet::new();
        let type_vars = type_vars
            .into_iter()
            .filter(|(ty, _)| seen.insert(*ty))
            .collect::<Vec<_>>();

        for (ty, name) in type_vars {
            self.report(
                ErrorBuilder::new()
                    .of_style(Style::Secondary)
                    .at_level(Level::Info)
                    .span(ty)
                    .continues()
                    .explain(format!("Call the type of {} `{}`.", name, ty))
                    .build()
            );
        }

        if let Some(immediate) = constraint.immediate() {
            self.report(
                ErrorBuilder::new()
                    .of_style(Style::Secondary)
                    .at_level(Level::Info)
                    .with_code(ErrorCode::UnificationFailure)
                    .message(format!(
                        "Error encountered while unifiying `{}` with `{}`",
                        immediate.expected, immediate.actual
                    ))
                    .span(immediate.expected)
                    .explain(format!("Expected `{}` here,", immediate.expected))
                    .build()
            );
            self.report(
                ErrorBuilder::new()
                    .of_style(Style::Secondary)
                    .at_level(Level::Info)
                    .with_code(ErrorCode::UnificationFailure)
                    .continues()
                    .span(immediate.actual)
                    .explain(format!(
                        "but received `{}` here.",
                        immediate.actual
                    ))
                    .build()
            );
        }
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

    /// The description should be a noun (phrase) preceded by a determiner such
    /// as "this".
    fn new_type_var<N: NodeInterface, NRef: AsRef<N>, S: AsRef<str>>(
        &mut self, source: NRef, description: Option<S>
    ) -> Handle<Type> {
        self.pool.generate(
            TypeValue::Var(
                self.gen.next(),
                description.map(|d| d.as_ref().to_string())
            ),
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
            ExprValue::BoundName(name) => {
                let Some(ty) = self.env.find(name.value.clone()) else {
                    self.report_unbound_name(*name);
                    return None;
                };
                *ty
            }
            ExprValue::MemberAccess(_, _) => todo!(),
            ExprValue::Call(_, _) => todo!(),
            ExprValue::ArrayLiteral(elements, should_continue) => {
                if elements.is_empty() && should_continue.is_some() {
                    self.new_type_var(
                        expr,
                        Some("this empty and size-indeterminate array literal")
                    )
                } else {
                    let element_count = elements.len();
                    let mut elements = elements.iter();
                    let element_type = if element_count > 0 {
                        let first = elements.next().unwrap();
                        let first_type = self.visit_expr(*first)?;
                        for other in elements {
                            let other_type = self.visit_expr(*other)?;
                            self.new_constraint(first_type, other_type);
                        }
                        first_type
                    } else {
                        self.new_type_var(
                            expr,
                            Some("the array literal's element")
                        )
                    };
                    let liquid_type = self.pool.generate(
                        if should_continue.is_some() {
                            LiquidTypeValue::All(
                                self.gen.next(),
                                Some("length".into())
                            )
                        } else {
                            LiquidTypeValue::Equal(element_count)
                        },
                        expr.start_token(),
                        expr.end_token()
                    );
                    self.pool.generate(
                        TypeValue::Array(element_type, liquid_type),
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
            ExprValue::InfixBop(lhs, op, rhs) => {
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
            ExprValue::PostfixBop(lhs, op, index, op2)
                if op.ty == TokenType::LeftBracket
                    && op2.ty == TokenType::RightBracket =>
            {
                let lhs_type = self.visit_expr(*lhs)?;
                let index_type = self.visit_expr(*index)?;

                // index must be an integer
                let op_index_type =
                    self.pool.generate(TypeValue::Int64, *op, *op2);
                self.new_constraint(op_index_type, index_type);

                // lhs must be an array with:
                // - some inner element type (`result_type`)
                // - some length (`index_liquid_type`)
                let result_type =
                    self.new_type_var(expr, Some("the array's element"));
                let lhs_size_liquid_type = self.pool.generate(
                    LiquidTypeValue::All(
                        self.gen.next(),
                        Some("length".into())
                    ),
                    lhs.start_token(),
                    lhs.end_token()
                );
                let actual_lhs_type = self.pool.generate(
                    TypeValue::Array(result_type, lhs_size_liquid_type),
                    lhs.start_token(),
                    lhs.end_token()
                );
                self.new_constraint(lhs_type, actual_lhs_type);
                result_type
            }
            _ => todo!()
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
            StmtValue::Assign(lhs, _, rhs) => {
                let lhs_type = self.visit_expr(*lhs)?;
                let rhs_type = self.visit_expr(*rhs)?;
                // todo: lhs_type is an lvalue
                self.new_constraint(lhs_type, rhs_type);
            }
            StmtValue::Divider(_) => todo!(),
            StmtValue::For {
                var,
                lower,
                exclusive_upper,
                body
            } => {
                self.env.push();
                let loop_var_type =
                    self.pool.generate(TypeValue::Int64, *var, *var);
                let lower_type = self.visit_expr(*lower)?;
                let upper_type = self.visit_expr(*exclusive_upper)?;
                // TODO: figure out better semantic arrangement/source for cnstr
                self.new_constraint(loop_var_type, lower_type);
                self.new_constraint(loop_var_type, upper_type);
                self.env.bind(var.value.clone(), loop_var_type);

                self.env.push();
                for stmt in body {
                    self.visit_stmt(*stmt)?;
                }
                self.env.pop();

                self.env.pop();
            }
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
}

impl<'pool, 'err, P: AsInferencePool> Unifier<Type, P>
    for TypeInferer<'pool, 'err, P>
{
    fn type_pool_mut(&mut self) -> &mut P {
        self.pool
    }

    fn constraints(&mut self) -> &mut VecDeque<Handle<TypeConstraint>> {
        &mut self.type_constraints
    }

    fn unify_constraint(
        &mut self, dsu: &mut DisjointSets<Handle<Type>>,
        constraint: Handle<TypeConstraint>
    ) -> Result<(), ()> {
        let expected_rep = dsu
            .find(constraint.expected)
            .expect("expected type not added to dsu");
        let actual_rep = dsu
            .find(constraint.actual)
            .expect("actual type not added to dsu");
        if expected_rep != actual_rep {
            match (&expected_rep.value, &actual_rep.value) {
                (TypeValue::Var(..), TypeValue::Var(..)) => {
                    dsu.union(actual_rep, expected_rep, false)
                        .expect("failed to union");
                }
                (TypeValue::Var(..), _) => {
                    dsu.union(expected_rep, actual_rep, false)
                        .expect("failed to union");
                }
                (_, TypeValue::Var(..)) => {
                    dsu.union(actual_rep, expected_rep, false)
                        .expect("failed to union");
                }
                _ if expected_rep.can_unify_with(&actual_rep) => {
                    dsu.union(actual_rep, expected_rep, false)
                        .expect("failed to union");
                    for (expected_subterm, actual_subterm) in
                        zip(expected_rep.subterms(), actual_rep.subterms())
                    {
                        self.derive_constraint(
                            expected_subterm,
                            actual_subterm,
                            constraint
                        );
                    }
                    for (expected_liquid_subterm, actual_liquid_subterm) in zip(
                        expected_rep.liquid_subterms(),
                        actual_rep.liquid_subterms()
                    ) {
                        self.new_constraint(
                            expected_liquid_subterm,
                            actual_liquid_subterm
                        );
                    }
                }
                _ => {
                    self.report_unification_failure(constraint, dsu, None);
                    return Err(());
                }
            }
        }
        Ok(())
    }
}

impl<'pool, 'err, P: AsInferencePool> Unifier<LiquidType, P>
    for TypeInferer<'pool, 'err, P>
{
    fn type_pool_mut(&mut self) -> &mut P {
        self.pool
    }

    fn constraints(&mut self) -> &mut VecDeque<Handle<LiquidTypeConstraint>> {
        &mut self.liquid_type_constraints
    }

    fn unify_constraint(
        &mut self, dsu: &mut DisjointSets<Handle<LiquidType>>,
        constraint: Handle<UnificationConstraint<LiquidType>>
    ) -> Result<(), ()> {
        let expected_rep = dsu
            .find(constraint.expected)
            .expect("expected liquid type not added to dsu");
        let actual_rep = dsu
            .find(constraint.actual)
            .expect("actual liquid type not added to dsu");
        if expected_rep != actual_rep {
            match (&expected_rep.value, &actual_rep.value) {
                (LiquidTypeValue::All(..), LiquidTypeValue::All(..)) => {
                    dsu.union(actual_rep, expected_rep, false)
                        .expect("failed to union");
                }
                (LiquidTypeValue::All(..), _) => {
                    dsu.union(expected_rep, actual_rep, false)
                        .expect("failed to union");
                }
                (_, LiquidTypeValue::All(..)) => {
                    dsu.union(actual_rep, expected_rep, false)
                        .expect("failed to union");
                }
                (LiquidTypeValue::Equal(a), LiquidTypeValue::Equal(b))
                    if a == b =>
                {
                    dsu.union(actual_rep, expected_rep, false);
                }
                _ => {
                    println!("{:?}", constraint);
                    self.report_unification_failure(constraint, dsu, None);
                    return Err(());
                }
            }
        }
        Ok(())
    }
}
