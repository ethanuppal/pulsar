//! Implements a standard Hindley-Milner type inference algorithm.
//!
//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::token::{Token, TokenType};
use crate::{
    ast::{
        expr::Expr,
        node::NodeInterface,
        stmt::{Stmt, StmtValue},
        stmt_ty::StmtType,
        ty::{LiquidType, Type, TypeValue},
        AsASTPool
    },
    attribute::Attribute
};
use pulsar_utils::{
    disjoint_sets::DisjointSets,
    environment::Environment,
    error::{Error, ErrorBuilder, ErrorCode, ErrorManager, Level, Style},
    loc::Span,
    pool::{AsPool, Handle}
};
use std::{
    collections::{HashMap, HashSet},
    iter::zip
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
    pub fn infer(mut self) -> Option<Vec<Stmt>> {
        for top_level in &self.ast {
            self.register_top_level_bindings(*top_level);
        }

        for stmt in &mut self.ast {
            self.visit_stmt(*stmt)?;
        }

        let substitution = self.unify_constraints()?;
        for (ty, sub_ty) in &substitution {
            match *sub_ty.cell.as_ref() {
                Type::Var(_) => {
                    self.report_ambiguous_type(
                        sub_ty.cell.clone(),
                        "Type variable not resolved (bug?)".into()
                    );
                    return None;
                }
                Type::Array(_, ARRAY_TYPE_UNKNOWN_SIZE) => {
                    self.report_ambiguous_type(
                        sub_ty.cell.clone(),
                        "Array size not resolved".into()
                    );
                    return None;
                }
                _ => {}
            }
            *ty.cell.as_mut() = sub_ty.get();
        }

        Some(program)
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
        &mut self, ty: TypeCell, /* expr: &Expr, */ explain: String
    ) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::StaticAnalysisIssue)
                // .at_region(&expr.start)
                .without_loc()
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
        &mut self, lhs: TypeCell, rhs: TypeCell, lhs_ctx: Span, rhs_ctx: Span,
        fix: Option<String>
    ) {
        let mut builder = ErrorBuilder::new()
            .of_style(Style::Primary)
            .at_level(Level::Error)
            .with_code(ErrorCode::StaticAnalysisIssue)
            .span(&lhs_ctx)
            .message(format!("Failed to unify types `{}` and `{}`", lhs, rhs))
            .explain(format!("Type inferred here to be `{}`", lhs));
        if let Some(fix) = fix {
            builder = builder.fix(fix);
        }
        self.report(builder.build());
        if lhs_ctx != rhs_ctx {
            self.report(
                ErrorBuilder::new()
                    .of_style(Style::Secondary)
                    .at_level(Level::Error)
                    .with_code(ErrorCode::StaticAnalysisIssue)
                    .span(&rhs_ctx)
                    .continues()
                    .explain(format!("Type inferred here to be `{}`", rhs))
                    .build()
            );
        }
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
                .map(|(_, arg_type)| self.ast_pool.duplicate(*arg_type))
                .collect();
            let ret_type = ret;
            let func_type = self.ast_pool.new(
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
        self.type_constraints
            .push(TypeConstraint::new(expected, actual));
    }

    fn derive_constraint(&mut self, source: Handle<TypeConstraint>) {
        self.type_constraints
            .push(TypeConstraint::derived(expected, actual, source));
    }

    fn new_type_var(&self) -> Type {
        Type::Var(Gen::next("TypeInferer::get_type_var"))
    }

    /// Extracts constraints from the expression `expr` and returns either a
    /// type variable or fully resolved type for it, along with a derivation of
    /// purity.
    fn visit_expr(&mut self, expr: Handle<Expr>) -> Option<Handle<Type>> {
        let mut expr_is_pure = true;
        match &expr.value {
            ExprValue::ConstantInt(_) => {
                *expr.ty.as_mut() = Type::Int64;
            }
            ExprValue::BoundName(name) => {
                if let Some(name_ty) = self.env.find(name.value.clone()) {
                    *expr.ty.as_mut() = self.new_type_var();
                    self.add_constraint(
                        expr.ty.clone(),
                        name_ty.clone(),
                        expr.span(),
                        name.span()
                    );
                } else {
                    self.report_unbound_name(name);
                    return None;
                }
            }
            ExprValue::Call(name, args) => {
                if let Some(name_ty) = self.env.find(name.value.clone()) {
                    // Not sure if I need to clone here
                    match name_ty.clone_out() {
                        Type::Function {
                            is_pure,
                            args: param_tys,
                            ret
                        } => {
                            expr_is_pure &= is_pure;

                            *expr.ty.as_mut() = self.new_type_var();
                            self.add_constraint(
                                expr.ty.clone(),
                                TypeCell::new((*ret).clone()),
                                expr.span(),
                                name.span()
                            );

                            for (arg, param_ty) in zip(args, param_tys) {
                                let (arg_ty, arg_is_pure) =
                                    self.visit_expr(arg)?;
                                expr_is_pure &= arg_is_pure;
                                self.add_constraint(
                                    arg_ty,
                                    TypeCell::new(param_ty),
                                    arg.span(),
                                    name.span() /* TODO: store the param
                                                 * tokens? */
                                )
                            }
                        }
                        _ => {
                            self.report_called_non_function(name);
                            return None;
                        }
                    }
                } else {
                    self.report_unbound_name(name);
                    return None;
                }
            }
            ExprValue::MemberAccess(_, _) => todo!(),
            ExprValue::PostfixBop(array, op1, index, op2)
                if op1.ty == TokenType::LeftBracket
                    && op2.ty == TokenType::RightBracket =>
            {
                *expr.ty.as_mut() = self.new_type_var();
                let (array_ty, array_is_pure) = self.visit_expr(array)?;
                let (index_ty, index_is_pure) = self.visit_expr(index)?;
                expr_is_pure &= array_is_pure && index_is_pure;
                self.add_constraint(
                    index_ty,
                    Type::int64_singleton(),
                    index.span(),
                    array.span()
                );
                self.add_constraint(
                    TypeCell::new(Type::Array(
                        expr.ty.clone(),
                        ARRAY_TYPE_UNKNOWN_SIZE
                    )),
                    array_ty,
                    expr.span(),
                    expr.span()
                );
                // match array_ty.clone_out() {
                //     Type::Array(element_type, _) => {

                //     }
                //     _ => {
                //         self.report_invalid_operation(
                //             "Subscript used on non-array type".into(),
                //             &index.start
                //         );
                //         return None;
                //     }
                // }
            }
            ExprValue::PostfixBop(_, _, _, _) => todo!(),
            ExprValue::ArrayLiteral(elements, should_continue) => {
                let element_ty_var = self.new_type_var();
                let element_ty_var_cell = TypeCell::new(element_ty_var);
                *expr.ty.as_mut() = Type::Array(
                    element_ty_var_cell.clone(),
                    if *should_continue {
                        ARRAY_TYPE_UNKNOWN_SIZE
                    } else {
                        elements
                            .len()
                            .try_into()
                            .unwrap_or_else(|_| panic!("how?"))
                    }
                );
                for element in elements {
                    let (element_type, element_is_pure) =
                        self.visit_expr(element)?;
                    expr_is_pure &= element_is_pure;
                    self.add_constraint(
                        element_ty_var_cell.clone(),
                        element_type,
                        expr.span(),
                        element.span()
                    );
                }
            }
            ExprValue::PrefixOp(_, _) => todo!(),
            ExprValue::InfixBop(lhs, bop, rhs) => {
                // for now we hard code it
                // i don't know how to deal with e.g. operator overloading here
                match bop.ty {
                    TokenType::Plus | TokenType::Minus | TokenType::Times => {
                        let (lhs_ty, lhs_is_pure) = self.visit_expr(lhs)?;
                        let (rhs_ty, rhs_is_pure) = self.visit_expr(rhs)?;
                        expr_is_pure &= lhs_is_pure && rhs_is_pure;

                        self.add_constraint(
                            expr.ty.clone(),
                            lhs_ty,
                            expr.span(),
                            lhs.span()
                        );
                        self.add_constraint(
                            expr.ty.clone(),
                            rhs_ty,
                            expr.span(),
                            rhs.span()
                        );

                        *expr.ty.as_mut() = Type::Int64;
                    }
                    _ => ()
                }
            }
            ExprValue::HardwareMap(map_token, _, f, arr) => {
                *expr.ty.as_mut() = self.new_type_var();
                let (arr_ty, arr_is_pure) = self.visit_expr(arr)?;
                expr_is_pure &= arr_is_pure;
                if let Some(f_type) = self.env.find(f.value.clone()) {
                    // f : pure (Int64) -> Int64
                    self.add_constraint(
                        f_type.clone(),
                        TypeCell::new(Type::Function {
                            is_pure: true,
                            args: vec![Type::Int64],
                            ret: Box::new(Type::Int64)
                        }),
                        f.span(),
                        map_token.span()
                    );
                    // arr_ty = Int64[?]
                    self.add_constraint(
                        arr_ty.clone(),
                        TypeCell::new(Type::Array(
                            Type::int64_singleton(),
                            ARRAY_TYPE_UNKNOWN_SIZE
                        )),
                        arr.span(),
                        map_token.span()
                    );
                    // expr.ty = arr_ty
                    self.add_constraint(
                        expr.ty.clone(),
                        arr_ty,
                        map_token.span(),
                        arr.span()
                    );
                } else {
                    self.report_unbound_name(f);
                    return None;
                }
            }
        };

        Some((expr.ty.clone(), expr_is_pure))
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
        &mut self, dsu: &mut DisjointSets<TypeNode>, lhs: TypeCell,
        rhs: TypeCell, lhs_ctx: Span, rhs_ctx: Span
    ) -> Result<(), String> {
        let lhs_tn = TypeNode::from_currently_stable_cell(lhs.clone());
        let rhs_tn = TypeNode::from_currently_stable_cell(rhs.clone());
        dsu.add(lhs_tn.clone());
        dsu.add(rhs_tn.clone());
        let lhs_r = dsu
            .find(lhs_tn)
            .ok_or_else(|| "dsu find failed".to_string())?;
        let rhs_r = dsu
            .find(rhs_tn)
            .ok_or_else(|| "dsu find failed".to_string())?;
        if lhs_r != rhs_r {
            match (lhs_r.get(), rhs_r.get()) {
                (Type::Var(_), Type::Var(_)) => {
                    dsu.union(lhs_r, rhs_r, true)
                        .ok_or_else(|| "dsu union failed".to_string())?;
                }
                (Type::Var(_), _) => {
                    dsu.union(lhs_r, rhs_r, false)
                        .ok_or_else(|| "dsu union failed".to_string())?;
                }
                (_, Type::Var(_)) => {
                    dsu.union(rhs_r, lhs_r, false)
                        .ok_or_else(|| "dsu union failed".to_string())?;
                }
                (
                    Type::Array(lhs_element_ty, lhs_size),
                    Type::Array(rhs_element_ty, rhs_size)
                ) => match (lhs_size, rhs_size) {
                    (ARRAY_TYPE_UNKNOWN_SIZE, ARRAY_TYPE_UNKNOWN_SIZE) => {
                        dsu.union(lhs_r, rhs_r, true)
                            .ok_or_else(|| "dsu union failed".to_string())?;
                        self.unify(
                            dsu,
                            lhs_element_ty,
                            rhs_element_ty,
                            lhs_ctx,
                            rhs_ctx
                        )?;
                    }
                    (ARRAY_TYPE_UNKNOWN_SIZE, _) => {
                        dsu.union(lhs_r, rhs_r, false)
                            .ok_or_else(|| "dsu union failed".to_string())?;
                        self.unify(
                            dsu,
                            lhs_element_ty,
                            rhs_element_ty,
                            lhs_ctx,
                            rhs_ctx
                        )?;
                    }
                    (_, ARRAY_TYPE_UNKNOWN_SIZE) => {
                        dsu.union(rhs_r, lhs_r, false)
                            .ok_or_else(|| "dsu union failed".to_string())?;
                        self.unify(
                            dsu,
                            rhs_element_ty,
                            lhs_element_ty,
                            lhs_ctx,
                            rhs_ctx
                        )?;
                    }
                    _ => {
                        if lhs_size != rhs_size {
                            self.report(
                                ErrorBuilder::new()
                                    .of_style(Style::Primary)
                                    .at_level(Level::Error)
                                    .with_code(ErrorCode::StaticAnalysisIssue)
                                    .span(&lhs_ctx)
                                    .message(format!(
                                        "Array sizes don't match: {} != {}",
                                        lhs_size, rhs_size
                                    ))
                                    .build()
                            );
                            self.report(
                                    ErrorBuilder::new()
                                        .of_style(Style::Secondary)
                                        .at_level(Level::Error)
                                        .with_code(ErrorCode::StaticAnalysisIssue)
                                        .span(&rhs_ctx)
                                        .message("...".into())
                                        .explain(format!("Inferred to have size {} here based on environment", rhs_size))
                                        .build()
                                );

                            return Err("array type error".into());
                        }
                    }
                },
                (
                    Type::Function {
                        is_pure: lhs_is_pure,
                        args: lhs_args,
                        ret: lhs_ret
                    },
                    Type::Function {
                        is_pure: rhs_is_pure,
                        args: rhs_args,
                        ret: rhs_ret
                    }
                ) => {
                    if !lhs_is_pure && rhs_is_pure {
                        self.report_unification_failure(
                            lhs,
                            rhs,
                            lhs_ctx.clone(),
                            lhs_ctx,
                            Some("Try marking the function as `pure`".into())
                        );
                        return Err("unification failure".into());
                    }
                    for (lhs_arg, rhs_arg) in zip(lhs_args, rhs_args) {
                        self.unify(
                            dsu,
                            TypeCell::new(lhs_arg),
                            TypeCell::new(rhs_arg),
                            lhs_ctx.clone(),
                            rhs_ctx.clone()
                        )?;
                    }
                    self.unify(
                        dsu,
                        TypeCell::new(*lhs_ret),
                        TypeCell::new(*rhs_ret),
                        lhs_ctx,
                        rhs_ctx
                    )?;
                }
                _ => {
                    self.report_unification_failure(
                        lhs, rhs, lhs_ctx, rhs_ctx, None
                    );
                    return Err("unification failure".into());
                }
            }
        }
        Ok(())
    }

    fn unify_constraints(&mut self) -> Option<DisjointSets<TypeNode>> {
        let mut dsu = DisjointSets::new();
        while !self.constraints.is_empty() {
            let constraint = self.constraints.pop_front()?;
            match constraint {
                TypeConstraint::Equality {
                    lhs,
                    rhs,
                    lhs_ctx,
                    rhs_ctx
                } => {
                    let _ = self
                        .unify(&mut dsu, lhs, rhs, lhs_ctx, rhs_ctx)
                        .map_err(|_| {
                            if !self.error_manager.borrow().has_errors() {
                                panic!(
                                    "TypeInferer failed without error message"
                                );
                            }
                        });
                }
            }
        }
        dsu.collapse();
        Some(dsu)
    }
}
