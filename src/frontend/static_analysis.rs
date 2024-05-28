use super::{
    ast::{Expr, ExprValue, Node, NodeValue},
    token::{Token, TokenType},
    ty::{
        StmtTermination, StmtType, StmtTypeCell, Type, TypeCell,
        ARRAY_TYPE_UNKNOWN_SIZE
    }
};
use crate::utils::{
    disjoint_set::{DisjointSets, NodeTrait},
    environment::Environment,
    error::{Error, ErrorBuilder, ErrorCode, ErrorManager, Level, Style},
    id::Gen,
    loc::Region,
    CheapClone
};
use std::{cell::RefCell, fmt::Debug, iter::zip, rc::Rc};

/// A dummy variable bound in the environment for the return type. No valid
/// identifier contains a space, so we are guaranteed to have no name
/// collisions.
const RETURN_ID: &str = " return";

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypeNode {
    cell: TypeCell
}

impl TypeNode {
    pub fn from_currently_stable_cell(cell: TypeCell) -> Self {
        Self { cell }
    }

    pub fn get(&self) -> Type {
        self.cell.clone_out()
    }
}

impl CheapClone for TypeNode {}
impl Debug for TypeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.cell.as_ref())
    }
}
impl NodeTrait for TypeNode {}

#[derive(Debug)]
struct TypeConstraint {
    pub lhs: TypeCell,
    pub rhs: TypeCell,
    pub lhs_ctx: Token,
    pub rhs_ctx: Token
}

pub struct StaticAnalyzer {
    env: Environment<String, TypeCell>,
    constraints: Vec<TypeConstraint>,
    error_manager: Rc<RefCell<ErrorManager>>
}

impl StaticAnalyzer {
    pub fn new(error_manager: Rc<RefCell<ErrorManager>>) -> StaticAnalyzer {
        StaticAnalyzer {
            env: Environment::new(),
            constraints: vec![],
            error_manager
        }
    }

    /// Establishes a top-level binding for the type `ty` of `name`, useful for
    /// allowing functions to call other functions or external/FFI declarations.
    pub fn bind_top_level(&mut self, name: String, ty: Type) {
        self.env.bind_base(name, TypeCell::new(ty));
    }

    /// Performs control-flow analysis on funcitions and infers the types of
    /// nodes and expression in the program `program`, returning the
    /// annotated AST if no error occured.
    pub fn infer(&mut self, mut program: Vec<Node>) -> Option<Vec<Node>> {
        self.constraints.clear();

        self.env.push();
        for node in &mut program {
            self.visit_node(node, true)?;
        }
        for node in &mut program {
            self.visit_node(node, false)?;
        }
        self.env.pop();

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
        self.error_manager.borrow_mut().record(error);
    }

    fn warn_dead_code(
        &mut self, func_name: &Token, dead_node: &Node, term_node: &Node
    ) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Warning)
                .with_code(ErrorCode::StaticAnalysisIssue)
                .at_region(dead_node.start(), 1)
                .message("Statement is never reached".into())
                .build()
        );
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Secondary)
                .at_level(Level::Warning)
                .at_region(term_node.start(), 1)
                .message("   ...".into())
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
                .at_token(func_name)
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
                .at_token(name)
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
                // .at_token(&expr.start)
                .without_loc()
                .message(format!("Ambiguous type `{}`", ty))
                .explain(explain)
                .build()
        );
    }

    fn report_failed_purity_derivation(
        &mut self, pure_token: &Token, name: &Token, impure_node: &Node
    ) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::StaticAnalysisIssue)
                .at_region(impure_node.start(), 1)
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
                .at_token(pure_token)
                .message("   ...".into())
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
                .at_token(name)
                .message(format!(
                    "Cannot call non-function value `{}`",
                    name.value
                ))
                .build()
        );
    }

    fn report_unification_failure(
        &mut self, lhs: TypeCell, rhs: TypeCell, lhs_ctx: Token,
        rhs_ctx: Token, fix: Option<String>
    ) {
        let mut builder = ErrorBuilder::new()
            .of_style(Style::Primary)
            .at_level(Level::Error)
            .with_code(ErrorCode::StaticAnalysisIssue)
            .at_token(&lhs_ctx)
            .message(format!("Failed to unify types `{}` and `{}`", lhs, rhs))
            .explain(format!("Type inferred here to be `{}`", lhs));
        if let Some(fix) = fix {
            builder = builder.fix(fix);
        }
        self.report(builder.build());
        if lhs_ctx.loc != rhs_ctx.loc {
            self.report(
                ErrorBuilder::new()
                    .of_style(Style::Secondary)
                    .at_level(Level::Error)
                    .with_code(ErrorCode::StaticAnalysisIssue)
                    .at_token(&rhs_ctx)
                    .message("   ...".into())
                    .explain(format!("Type inferred here to be `{}`", rhs))
                    .build()
            );
        }
    }
}

impl StaticAnalyzer {
    fn new_type_var(&self) -> Type {
        Type::Var(Gen::next("TypeInferer::get_type_var"))
    }

    fn add_constraint(
        &mut self, lhs: TypeCell, rhs: TypeCell, lhs_ctx: Token, rhs_ctx: Token
    ) {
        self.constraints.push(TypeConstraint {
            lhs,
            rhs,
            lhs_ctx,
            rhs_ctx
        });
    }

    /// Extracts constraints from the expression `expr` and returns either a
    /// type variable or fully resolved type for it, along with a derivation of
    /// purity.
    fn visit_expr(&mut self, expr: &Expr) -> Option<(TypeCell, bool)> {
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
                        expr.start.clone(),
                        name.clone()
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
                                expr.start.clone(),
                                name.clone()
                            );

                            for (arg, param_ty) in zip(args, param_tys) {
                                let (arg_ty, arg_is_pure) =
                                    self.visit_expr(arg)?;
                                expr_is_pure &= arg_is_pure;
                                self.add_constraint(
                                    arg_ty,
                                    TypeCell::new(param_ty),
                                    arg.start.clone(),
                                    name.clone() /* TODO: store the param
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
                        expr.start.clone(),
                        element.start.clone()
                    );
                }
            }
            ExprValue::PrefixOp(_, _) => todo!(),
            ExprValue::BinOp(lhs, bop, rhs) => {
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
                            expr.start.clone(),
                            lhs.start.clone()
                        );
                        self.add_constraint(
                            expr.ty.clone(),
                            rhs_ty,
                            expr.start.clone(),
                            rhs.start.clone()
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
                        f.clone(),
                        map_token.clone()
                    );
                    // arr_ty = Int64[?]
                    self.add_constraint(
                        arr_ty.clone(),
                        TypeCell::new(Type::Array(
                            Type::int64_singleton(),
                            ARRAY_TYPE_UNKNOWN_SIZE
                        )),
                        arr.start.clone(),
                        map_token.clone()
                    );
                    // expr.ty = arr_ty
                    self.add_constraint(
                        expr.ty.clone(),
                        arr_ty,
                        map_token.clone(),
                        arr.start.clone()
                    );
                } else {
                    self.report_unbound_name(f);
                    return None;
                }
            }
        };

        Some((expr.ty.clone(), expr_is_pure))
    }

    fn visit_node(
        &mut self, node: &Node, top_level_pass: bool
    ) -> Option<StmtTypeCell> {
        match (&node.value, top_level_pass) {
            (
                NodeValue::Function {
                    name,
                    params,
                    ret,
                    pure_token,
                    body: _
                },
                true
            ) => {
                // On top-level pass, bind all functions to their types
                self.env.bind(
                    name.value.clone(),
                    TypeCell::new(Type::Function {
                        is_pure: pure_token.is_some(),
                        args: params
                            .iter()
                            .map(|p| p.1.clone())
                            .collect::<Vec<_>>(),
                        ret: Box::new(ret.clone())
                    })
                );
                // since we don't know termination, assume nonterminal
                *node.ty.as_mut() = StmtType::from(
                    StmtTermination::Nonterminal,
                    pure_token.is_some()
                );
            }
            (
                NodeValue::Function {
                    name,
                    params,
                    ret,
                    pure_token,
                    body
                },
                false
            ) => {
                self.env.push();
                self.env.bind(RETURN_ID.into(), TypeCell::new(ret.clone()));
                for (name, ty) in params {
                    self.env
                        .bind(name.value.clone(), TypeCell::new(ty.clone()));
                }

                let func_ty = node.ty.clone();
                let mut warned_dead_code = false;
                let mut term_node = None;
                for node in body {
                    let node_ty = self.visit_node(node, false)?;
                    let mut just_found_term = false;
                    if node_ty.as_ref().termination == StmtTermination::Terminal
                        && term_node.is_none()
                    {
                        term_node = Some(node);
                        func_ty.as_mut().termination =
                            StmtTermination::Terminal;
                        just_found_term = true;
                    }
                    if func_ty.as_ref().termination == StmtTermination::Terminal
                        && !warned_dead_code
                        && !just_found_term
                        && !node.is_phantom()
                    {
                        self.warn_dead_code(name, node, term_node.unwrap());
                        warned_dead_code = true;
                    }
                    if !node_ty.as_ref().is_pure && pure_token.is_some() {
                        self.report_failed_purity_derivation(
                            &pure_token.clone().unwrap(),
                            name,
                            node
                        );
                        return None;
                    }
                }
                if func_ty.as_ref().termination == StmtTermination::Nonterminal
                {
                    self.report_missing_return(name);
                    return None;
                }
                self.env.pop();
            }
            (
                NodeValue::LetBinding {
                    name,
                    hint: hint_opt,
                    value
                },
                false
            ) => {
                let (value_ty, expr_is_pure) = self.visit_expr(&value)?;
                if let Some(hint) = hint_opt {
                    self.add_constraint(
                        hint.clone(),
                        value_ty.clone(),
                        name.clone(),
                        value.start.clone()
                    );
                }
                self.env.bind(name.value.clone(), value_ty);
                // TODO: never types
                *node.ty.as_mut() =
                    StmtType::from(StmtTermination::Nonterminal, expr_is_pure);
            }
            (
                NodeValue::Return {
                    token,
                    value: value_opt
                },
                false
            ) => {
                let ((value_ty, value_is_pure), value_start) =
                    if let Some(value) = value_opt {
                        (self.visit_expr(value)?, value.start.clone())
                    } else {
                        ((Type::unit_singleton(), true), token.clone())
                    };
                self.add_constraint(
                    value_ty.clone(),
                    self.env.find(RETURN_ID.into()).unwrap().clone(),
                    value_start,
                    token.clone()
                );
                *node.ty.as_mut() =
                    StmtType::from(StmtTermination::Terminal, value_is_pure);
            }
            _ => {}
        }
        Some(node.ty.clone())
    }

    /// Whenever possible, pass `lhs` as the type to be unified into `rhs`.
    fn unify(
        &mut self, dsu: &mut DisjointSets<TypeNode>, lhs: TypeCell,
        rhs: TypeCell, lhs_ctx: Token, rhs_ctx: Token
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
                                    .at_token(&lhs_ctx)
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
                                        .at_token(&rhs_ctx)
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
                            lhs_ctx.clone(),
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
            let constraint = self.constraints.pop()?;
            let _ = self
                .unify(
                    &mut dsu,
                    constraint.lhs,
                    constraint.rhs,
                    constraint.lhs_ctx,
                    constraint.rhs_ctx
                )
                .map_err(|_| {
                    if !self.error_manager.borrow().has_errors() {
                        panic!("TypeInferer failed without error message");
                    }
                });
        }
        dsu.collapse();
        Some(dsu)
    }
}