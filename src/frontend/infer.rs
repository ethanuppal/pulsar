use super::{
    ast::{Expr, ExprValue, Node, NodeValue},
    token::{Token, TokenType},
    ty::{Type, ARRAY_TYPE_UNKNOWN_SIZE}
};
use crate::utils::{
    context::{Context, Name},
    dsu::{DisjointSets, NodeTrait},
    error::{Error, ErrorBuilder, ErrorCode, ErrorManager, Level, Style},
    id::Gen,
    CheapClone
};
use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Clone)]
struct TypeNode {
    pointer: Rc<RefCell<Type>>
}

impl TypeNode {
    fn from_currently_unchanging(pointer: Rc<RefCell<Type>>) -> Self {
        Self { pointer }
    }
}

impl PartialEq for TypeNode {
    fn eq(&self, other: &Self) -> bool {
        self.pointer.borrow().eq(&other.pointer.borrow())
    }
}
impl Eq for TypeNode {}
impl Hash for TypeNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pointer.borrow().hash(state)
    }
}
impl CheapClone for TypeNode {}
impl NodeTrait for TypeNode {}

pub struct TypeConstraint {
    pub lhs: Rc<RefCell<Type>>,
    pub rhs: Rc<RefCell<Type>>
}

pub struct TypeInferer {
    env: Context<Rc<RefCell<Type>>>,
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
        self.env.bind_base(name, Rc::new(RefCell::new(ty)));
    }

    /// Infers the types of nodes and expression in the program `program`.
    /// Returns `false` on error.
    pub fn infer(&mut self, mut program: Vec<Node>) -> bool {
        self.constraints.clear();
        self.env.push();
        for node in &mut program {
            if self.visit_node(node, true).is_none() {
                return false;
            }
        }
        for node in &mut program {
            if self.visit_node(node, false).is_none() {
                return false;
            }
        }
        if self.unify_constraints().is_none() {
            return false;
        }
        self.env.pop();
        true
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
}

impl TypeInferer {
    fn new_type_var(&self) -> Type {
        Type::Var(Gen::next("TypeInferer::get_type_var".into()))
    }

    fn add_constraint(
        &mut self, lhs: Rc<RefCell<Type>>, rhs: Rc<RefCell<Type>>
    ) {
        self.constraints.push(TypeConstraint { lhs, rhs });
    }

    /// Extracts constraints from the expression `expr` and returns either a
    /// type variable or fully resolved type for it.
    fn visit_expr(&mut self, expr: &Expr) -> Option<Rc<RefCell<Type>>> {
        match &expr.value {
            ExprValue::ConstantInt(_) => {
                *expr.ty.borrow_mut() = Type::Int64;
            }
            ExprValue::BoundName(name) => {
                if let Some(name_ty) = self.env.find(name.value.clone()) {
                    *expr.ty.borrow_mut() = self.new_type_var();
                    self.add_constraint(expr.ty.clone(), name_ty.clone());
                } else {
                    self.report_unbound_name(name);
                    return None;
                }
            }
            ExprValue::ArrayLiteral(elements, _) => {
                let element_ty_var = self.new_type_var();
                let element_ty_unsure =
                    Rc::new(RefCell::new(element_ty_var.clone()));
                *expr.ty.borrow_mut() = Type::Array(
                    Box::new(element_ty_var),
                    ARRAY_TYPE_UNKNOWN_SIZE
                );
                for element in elements {
                    let element_type = self.visit_expr(element)?;
                    self.add_constraint(
                        element_ty_unsure.clone(),
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

                        *expr.ty.borrow_mut() = Type::Int64;
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
                    Rc::new(RefCell::new(Type::Function {
                        is_pure: *is_pure,
                        args: params
                            .iter()
                            .map(|p| p.1.clone())
                            .collect::<Vec<_>>(),
                        ret: Box::new(ret.clone())
                    }))
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
                    self.env.bind(
                        name.value.clone(),
                        Rc::new(RefCell::new(ty.clone()))
                    );
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
        &mut self, dsu: &mut DisjointSets<TypeNode>, lhs: Rc<RefCell<Type>>,
        rhs: Rc<RefCell<Type>>
    ) -> Option<()> {
        Some(())
    }

    fn unify_constraints(&mut self) -> Option<()> {
        let mut dsu = DisjointSets::new();
        for constraint in &self.constraints {
            println!(
                "constraint: {} = {}",
                constraint.lhs.borrow(),
                constraint.rhs.borrow()
            );
        }
        if true {
            todo!("Need to use Arc<Mutex<T>> so that there's only one of each canonical type instance. actually how are you gonna do this.")
        }
        while !self.constraints.is_empty() {
            let constraint = self.constraints.pop()?;
            self.unify(&mut dsu, constraint.lhs, constraint.rhs)?;
        }
        Some(())
    }
}
