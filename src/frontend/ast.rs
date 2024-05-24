use super::token::Token;
use super::ty::{StmtType, Type, TypeCell};
use crate::utils::format;
use std::cell::RefCell;
use std::fmt;
use std::fmt::Display;
use std::rc::Rc;

pub type Param = (Token, Type);

#[derive(Clone)]
pub enum ExprValue {
    ConstantInt(i64),
    BoundName(Token),

    /// `ArrayLiteral(elements, should_continue)` is an array literal beginning
    /// with `elements` and filling the remainder of the array with zeros if
    /// `should_continue`.
    ArrayLiteral(Vec<Expr>, bool),
    PrefixOp(Token, Box<Expr>),
    BinOp(Box<Expr>, Token, Box<Expr>)
}

#[derive(Clone)]
pub struct Expr {
    pub value: ExprValue,
    pub ty: TypeCell
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ExprValue::ConstantInt(i) => {
                write!(f, "{}", i)?;
            }
            ExprValue::BoundName(name) => {
                write!(f, "{}", name.value)?;
            }
            ExprValue::ArrayLiteral(elements, should_continue) => {
                write!(
                    f,
                    "[{}{}]",
                    elements
                        .iter()
                        .map(|ty| ty.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    if *should_continue {
                        format!(
                            "{}...",
                            if elements.is_empty() { "" } else { ", " }
                        )
                    } else {
                        "".into()
                    }
                )?;
            }
            ExprValue::PrefixOp(op, rhs) => {
                write!(f, "({} {})", op.value, rhs)?;
            }
            ExprValue::BinOp(lhs, op, rhs) => {
                write!(f, "({} {} {})", lhs, op.value, rhs)?;
            }
        }

        let expr_ty = self.ty.as_ref();
        if expr_ty.clone().is_known() {
            write!(f, ": {}", expr_ty)?;
        }

        Ok(())
    }
}

#[derive(Clone)]
pub enum NodeValue {
    Function {
        name: Token,
        params: Vec<Param>,
        ret: Type,
        is_pure: bool,
        body: Vec<Node>
    },
    LetBinding {
        name: Token,
        hint: Option<TypeCell>,
        value: Box<Expr>
    }
}

#[derive(Clone)]
pub struct Node {
    pub value: NodeValue,
    pub ty: Rc<RefCell<StmtType>>
}

impl Node {
    fn pretty(&self, level: usize) -> String {
        let mut result = format::make_indent(level);
        let content = match &self.value {
            NodeValue::Function {
                name,
                params,
                ret,
                is_pure,
                body
            } => {
                let insert_newline = if body.is_empty() { "" } else { "\n" };
                format!(
                    "{}func {}({}) -> {} {{{}{}{}{}}}",
                    if *is_pure { "pure " } else { "" },
                    name.value,
                    params
                        .iter()
                        .map(|(name, ty)| format!("{}: {}", name.value, ty))
                        .collect::<Vec<_>>()
                        .join(", "),
                    ret,
                    insert_newline,
                    body.iter()
                        .map(|node| { node.pretty(level + 1) })
                        .collect::<Vec<_>>()
                        .join("\n"),
                    insert_newline,
                    format::make_indent(level)
                )
            }
            NodeValue::LetBinding {
                name,
                hint: hint_opt,
                value
            } => {
                let hint_str = if let Some(hint) = hint_opt {
                    format!(": {}", hint)
                } else {
                    "".into()
                };
                format!("let {}{} = {}", name.value, hint_str, value)
            }
        };
        result.push_str(&content);
        result
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty(0).fmt(f)
    }
}
