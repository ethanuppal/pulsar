use super::token::{Name, Token, TokenType};
use super::ty::{StmtType, Type};
use crate::utils::format;
use std::fmt::Display;
use std::fmt::{self, write};

pub type Param = (String, Type);

pub enum ExprValue {
    ConstantInt(i64),
    BinOp(Box<Expr>, TokenType, Box<Expr>)
}

pub struct Expr {
    pub value: ExprValue,
    pub ty: Option<Type>
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            ExprValue::ConstantInt(i) => write!(f, "{}", i),
            ExprValue::BinOp(left, op, right) => {
                write!(f, "({} {} {})", left, op, right)
            }
        }
    }
}

pub enum NodeValue {
    Function { name: String, body: Vec<Node> },
    LetBinding { name: String, value: Box<Expr> }
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
                    name,
                    insert_newline,
                    body.iter()
                        .map(|node| { node.pretty(level + 1) })
                        .collect::<Vec<_>>()
                        .join("\n"),
                    insert_newline,
                    format::make_indent(level)
                )
            }
            NodeValue::LetBinding { name, value } => {
                format!("let {} = {}", name, value)
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
