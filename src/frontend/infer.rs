use super::{
    ast::{Expr, ExprValue, Node, NodeValue},
    token::{Token, TokenType},
    ty::{Type, TypeCell, ARRAY_TYPE_UNKNOWN_SIZE}
};
use crate::utils::{
    context::{Context, Name},
    dsu::{DisjointSets, NodeTrait},
    error::{Error, ErrorBuilder, ErrorCode, ErrorManager, Level, Style},
    id::Gen,
    CheapClone
};
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypeNode {
    cell: TypeCell
}

impl TypeNode {
    pub fn from_currently_stable_cell(cell: TypeCell) -> Self {
        Self { cell }
    }

    pub fn get(&self) -> Type {
        self.cell.get()
    }
}

impl CheapClone for TypeNode {}
impl Debug for TypeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.cell.fmt(f)
    }
}
impl NodeTrait for TypeNode {}

pub struct TypeConstraint {
    pub lhs: TypeCell,
    pub rhs: TypeCell
}

pub struct TypeInferer {
    env: Context<TypeCell>,
    constraints: Vec<TypeConstraint>,
    error_manager: Rc<RefCell<ErrorManager>>
}

impl TypeInferer {
    pub fn new(error_manager: Rc<RefCell<ErrorManager>>) -> TypeInferer {
        TypeInferer {
            env: Context::new(),
            constraints: vec![],
            error_manager
        }
    }

    /// Establishes a top-level binding for the type `ty` of `name`, useful for
    /// allowing functions to call other functions or external/FFI declarations.
    pub fn bind_top_level(&mut self, name: Name, ty: Type) {
        self.env.bind_base(name, TypeCell::new(ty));
    }

    /// Infers the types of nodes and expression in the program `program`,
    /// returning the annotated AST if no error occured.
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
            *ty.cell.as_mut() = sub_ty.get();
        }

        Some(program)
    }

    fn report(&mut self, error: Error) {
        self.error_manager.borrow_mut().record(error);
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

    fn report_unification_failure(&mut self, lhs: TypeCell, rhs: TypeCell) {
        self.report(
            ErrorBuilder::new()
                .of_style(Style::Primary)
                .at_level(Level::Error)
                .with_code(ErrorCode::TypeError)
                .without_loc() // for now
                .message(format!(
                    "Failed to unify types `{}` and `{}`",
                    lhs, rhs
                ))
                .build()
        );
    }
}

impl TypeInferer {
    fn new_type_var(&self) -> Type {
        Type::Var(Gen::next("TypeInferer::get_type_var".into()))
    }

    fn add_constraint(&mut self, lhs: TypeCell, rhs: TypeCell) {
        self.constraints.push(TypeConstraint { lhs, rhs });
    }

    /// Extracts constraints from the expression `expr` and returns either a
    /// type variable or fully resolved type for it.
    fn visit_expr(&mut self, expr: &Expr) -> Option<TypeCell> {
        match &expr.value {
            ExprValue::ConstantInt(_) => {
                *expr.ty.as_mut() = Type::Int64;
            }
            ExprValue::BoundName(name) => {
                if let Some(name_ty) = self.env.find(name.value.clone()) {
                    *expr.ty.as_mut() = self.new_type_var();
                    self.add_constraint(expr.ty.clone(), name_ty.clone());
                } else {
                    self.report_unbound_name(name);
                    return None;
                }
            }
            ExprValue::ArrayLiteral(elements, _) => {
                let element_ty_var = self.new_type_var();
                let element_ty_var_cell = TypeCell::new(element_ty_var);
                *expr.ty.as_mut() = Type::Array(
                    element_ty_var_cell.clone(),
                    ARRAY_TYPE_UNKNOWN_SIZE
                );
                for element in elements {
                    let element_type = self.visit_expr(element)?;
                    self.add_constraint(
                        element_ty_var_cell.clone(),
                        element_type
                    );
                }
            }
            ExprValue::PrefixOp(_, _) => todo!(),
            ExprValue::BinOp(lhs, bop, rhs) => {
                // for now we hard code it
                // i don't know how to deal with e.g. operator overloading here
                match bop.ty {
                    TokenType::Plus | TokenType::Minus | TokenType::Times => {
                        let lhs_ty = self.visit_expr(lhs)?;
                        let rhs_ty = self.visit_expr(rhs)?;

                        self.add_constraint(expr.ty.clone(), lhs_ty);
                        self.add_constraint(expr.ty.clone(), rhs_ty);

                        *expr.ty.as_mut() = Type::Int64;
                    }
                    _ => ()
                }
            }
        };

        Some(expr.ty.clone())
    }

    fn visit_node(&mut self, node: &Node, top_level_pass: bool) -> Option<()> {
        match (&node.value, top_level_pass) {
            (
                NodeValue::Function {
                    name,
                    params,
                    ret,
                    is_pure,
                    body: _
                },
                true
            ) => {
                // On top-level pass, bind all functions to their types
                self.env.bind(
                    name.value.clone(),
                    TypeCell::new(Type::Function {
                        is_pure: *is_pure,
                        args: params
                            .iter()
                            .map(|p| p.1.clone())
                            .collect::<Vec<_>>(),
                        ret: Box::new(ret.clone())
                    })
                );
            }
            (
                NodeValue::Function {
                    name: _,
                    params,
                    ret: _,
                    is_pure: _,
                    body
                },
                false
            ) => {
                self.env.push();
                for (name, ty) in params {
                    self.env
                        .bind(name.value.clone(), TypeCell::new(ty.clone()));
                }
                for node in body {
                    self.visit_node(node, false)?;
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
                let value_ty = self.visit_expr(&value)?;
                if let Some(hint) = hint_opt {
                    self.add_constraint(hint.clone(), value_ty.clone());
                }
                self.env.bind(name.value.clone(), value_ty);
            }
            _ => {}
        }

        Some(())
    }

    fn unify(
        &mut self, dsu: &mut DisjointSets<TypeNode>, lhs: TypeCell,
        rhs: TypeCell
    ) -> Option<()> {
        let lhs_tn = TypeNode::from_currently_stable_cell(lhs.clone());
        let rhs_tn = TypeNode::from_currently_stable_cell(rhs.clone());
        dsu.add(lhs_tn.clone());
        dsu.add(rhs_tn.clone());
        let lhs_r = dsu.find(lhs_tn)?;
        let rhs_r = dsu.find(rhs_tn)?;
        if lhs_r != rhs_r {
            match (lhs_r.get(), rhs_r.get()) {
                (Type::Var(_), Type::Var(_)) => {
                    dsu.union(lhs_r, rhs_r, true)?;
                }
                (Type::Var(_), _) => {
                    dsu.union(lhs_r, rhs_r, false)?;
                }
                (_, Type::Var(_)) => {
                    dsu.union(rhs_r, lhs_r, false)?;
                }
                // TODO: impl when they are the same type constructor
                // in which case we need to deal with subterms and unify those
                // but like we'd need to make ref cells for those ig??
                (
                    Type::Array(lhs_element_ty, lhs_size),
                    Type::Array(rhs_element_ty, rhs_size)
                ) => {
                    match (lhs_size, rhs_size) {
                        (ARRAY_TYPE_UNKNOWN_SIZE, ARRAY_TYPE_UNKNOWN_SIZE) => {
                            todo!("fail here")
                        }
                        (ARRAY_TYPE_UNKNOWN_SIZE, _) => {
                            dsu.union(lhs_r, rhs_r, false)?;
                        }
                        (_, ARRAY_TYPE_UNKNOWN_SIZE) => {
                            dsu.union(rhs_r, lhs_r, false)?;
                        }
                        _ => {
                            if lhs_size != rhs_size {
                                self.report(
                                    ErrorBuilder::new()
                                        .of_style(Style::Primary)
                                        .at_level(Level::Error)
                                        .with_code(ErrorCode::TypeError)
                                        .without_loc()
                                        .message(format!(
                                            "Array sizes don't match: {} != {}",
                                            lhs_size, rhs_size
                                        ))
                                        .build()
                                );
                                return None;
                            }
                        }
                    }
                    self.unify(dsu, lhs_element_ty, rhs_element_ty)?;
                }
                _ => {
                    self.report_unification_failure(lhs, rhs);
                    return None;
                }
            }
        }
        Some(())
    }

    fn unify_constraints(&mut self) -> Option<DisjointSets<TypeNode>> {
        let mut dsu = DisjointSets::new();
        while !self.constraints.is_empty() {
            let constraint = self.constraints.pop()?;
            self.unify(&mut dsu, constraint.lhs, constraint.rhs)?;
        }
        Some(dsu)
    }
}
