use super::token::{Name, Token};
use super::ty::{StmtType, Type};
use crate::utils::format;
use std::fmt;
use std::fmt::Display;

pub type Param = (String, Type);

pub enum ExprValue {
    ConstantInt(i64),

    /// `ArrayLiteral(elements, should_continue)` is an array literal beginning
    /// with `elements` and filling the remainder of the array with zeros if
    /// `should_continue`.
    ArrayLiteral(Vec<Expr>, bool),
    PrefixOp(Token, Box<Expr>),
    BinOp(Box<Expr>, Token, Box<Expr>)
}

pub struct Expr {
    pub value: ExprValue,
    pub ty: Option<Type>
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ExprValue::ConstantInt(i) => {
                write!(f, "{}", i)?;
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
        if let Some(ty) = &self.ty {
            write!(f, ": {}", ty)?;
        }
        Ok(())
    }
}

pub enum NodeValue {
    Function {
        name: Name,
        body: Vec<Node>
    },
    LetBinding {
        name: Name,
        hint: Option<Type>,
        value: Box<Expr>
    }
}

pub struct Node {
    pub value: NodeValue,
    pub ty: Option<StmtType>
}

impl Node {
    fn pretty(&self, level: usize) -> String {
        let mut result = format::make_indent(level);
        let content = match &self.value {
            NodeValue::Function { name, body } => {
                let insert_newline = if body.is_empty() { "" } else { "\n" };
                format!(
                    "func {}() {{{}{}{}{}}}",
                    name.value,
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
