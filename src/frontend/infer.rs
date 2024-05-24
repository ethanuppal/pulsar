use super::{
    ast::{Expr, ExprValue, Node, NodeValue},
    ty::Type
};
use crate::utils::{
    context::{Context, Name},
    error::{Error, ErrorManager}
};
use std::cell::RefCell;
use std::rc::Rc;

pub struct TypeConstraint {
    pub lhs: Rc<RefCell<Type>>,
    pub rhs: Rc<RefCell<Type>>
}

pub struct TypeInferer {
    env: Context<Type>,
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
        self.env.bind_base(name, ty);
    }

    pub fn infer(&mut self, mut program: Vec<Node>) {
        self.constraints.clear();
        self.env.push();
        for node in &mut program {
            self.visit_node(node, true);
        }
        for node in &mut program {
            self.visit_node(node, false);
        }
        self.env.pop();
    }

    fn report(&mut self, error: Error) {
        self.error_manager.borrow_mut().record(error);
    }
}

impl TypeInferer {
    // pub fn visit_func()

    fn add_constraint(
        &mut self, lhs: Rc<RefCell<Type>>, rhs: Rc<RefCell<Type>>
    ) {
        self.constraints.push(TypeConstraint { lhs, rhs });
    }

    fn visit_expr(&mut self, expr: &Expr) -> Rc<RefCell<Type>> {
        match &expr.value {
            ExprValue::ConstantInt(_) => {
                *expr.ty.borrow_mut() = Type::Int64;
            }
            ExprValue::ArrayLiteral(_, _) => todo!(),
            ExprValue::PrefixOp(_, _) => todo!(),
            ExprValue::BinOp(_, _, _) => todo!()
        };

        // We have assigned a non-`None` value to `expr.ty` in every branch, so
        // we can safely unwrap; cloning an `Rc` doesn't do anything.
        expr.ty.clone()
    }

    fn visit_node(&mut self, node: &Node, top_level_pass: bool) {
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
                self.env.bind(
                    name.value.clone(),
                    Type::Function {
                        is_pure: *is_pure,
                        args: params
                            .iter()
                            .map(|p| p.1.clone())
                            .collect::<Vec<_>>(),
                        ret: Box::new(ret.clone())
                    }
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
                    self.env.bind(name.value.clone(), ty.clone());
                }
                for node in body {
                    self.visit_node(node, false)
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
                let value_ty = self.visit_expr(&value);
                if let Some(hint) = hint_opt {
                    self.add_constraint(hint.clone(), value_ty);
                }
            }
            _ => {}
        }
    }
}
