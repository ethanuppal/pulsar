//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::{node::Node, pretty_print::PrettyPrint, ty::Type};
use crate::token::Token;
use inform::fmt::IndentFormatter;
use pulsar_utils::pool::Handle;
use std::{
    fmt::{self, Display, Write},
    hash::{self, Hash}
};

#[derive(Clone)]
pub enum ExprValue {
    ConstantInt(i64),
    /// TODO: Support `::`s
    BoundName(Handle<Token>),

    MemberAccess(Handle<Expr>, Handle<Token>),

    /// TODO: Call an `expr` or some sort of chaining of `::`
    Call(Handle<Token>, Vec<Handle<Expr>>),

    /// `ArrayLiteral(elements, should_continue)` is an array literal beginning
    /// with `elements` and filling the remainder of the array with zeros if
    /// `should_continue.is_some()`.
    ArrayLiteral(Vec<Handle<Expr>>, Option<Handle<Token>>),

    PrefixOp(Handle<Token>, Handle<Expr>),
    InfixBop(Handle<Expr>, Handle<Token>, Handle<Expr>),
    PostfixBop(Handle<Expr>, Handle<Token>, Handle<Expr>, Handle<Token>)
}

impl PartialEq for ExprValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ExprValue::ConstantInt(a), ExprValue::ConstantInt(b)) => a == b,
            (ExprValue::BoundName(a), ExprValue::BoundName(b)) => {
                a.value == b.value
            }
            (
                ExprValue::MemberAccess(a_expr, a_token),
                ExprValue::MemberAccess(b_expr, b_token)
            ) => a_expr == b_expr && a_token.value == b_token.value,
            (
                ExprValue::Call(a_token, a_vec),
                ExprValue::Call(b_token, b_vec)
            ) => a_token.value == b_token.value && a_vec == b_vec,
            (
                ExprValue::ArrayLiteral(a_vec, a_opt),
                ExprValue::ArrayLiteral(b_vec, b_opt)
            ) => {
                a_vec == b_vec
                    && a_opt.map(|t| t.value.clone())
                        == b_opt.map(|t| t.value.clone())
            }
            (
                ExprValue::PrefixOp(a_token, a_expr),
                ExprValue::PrefixOp(b_token, b_expr)
            ) => a_token.value == b_token.value && a_expr == b_expr,
            (
                ExprValue::InfixBop(a_expr1, a_token, a_expr2),
                ExprValue::InfixBop(b_expr1, b_token, b_expr2)
            ) => {
                a_expr1 == b_expr1
                    && a_token.value == b_token.value
                    && a_expr2 == b_expr2
            }
            (
                ExprValue::PostfixBop(a_expr1, a_token, a_expr2, a_token2),
                ExprValue::PostfixBop(b_expr1, b_token, b_expr2, b_token2)
            ) => {
                a_expr1 == b_expr1
                    && a_token.value == b_token.value
                    && a_expr2 == b_expr2
                    && a_token2.value == b_token2.value
            }
            _ => false
        }
    }
}

impl Eq for ExprValue {}

impl Hash for ExprValue {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        match self {
            ExprValue::ConstantInt(a) => a.hash(state),
            ExprValue::BoundName(a) => a.value.hash(state),
            ExprValue::MemberAccess(a_expr, a_token) => {
                a_expr.hash(state);
                a_token.value.hash(state);
            }
            ExprValue::Call(a_token, a_vec) => {
                a_token.value.hash(state);
                a_vec.hash(state);
            }
            ExprValue::ArrayLiteral(a_vec, a_opt) => {
                a_vec.hash(state);
                if let Some(a_token) = a_opt {
                    a_token.value.hash(state);
                }
            }
            ExprValue::PrefixOp(a_token, a_expr) => {
                a_token.value.hash(state);
                a_expr.hash(state);
            }
            ExprValue::InfixBop(a_expr1, a_token, a_expr2) => {
                a_expr1.hash(state);
                a_token.value.hash(state);
                a_expr2.hash(state);
            }
            ExprValue::PostfixBop(a_expr1, a_token, a_expr2, a_token2) => {
                a_expr1.hash(state);
                a_token.value.hash(state);
                a_expr2.hash(state);
                a_token2.value.hash(state);
            }
        }
    }
}

pub type Expr = Node<ExprValue, Handle<Type>>;

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl PrettyPrint for Expr {
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result {
        match self.value {
            ExprValue::ConstantInt(i) => {
                write!(f, "{}", i)?;
            }
            ExprValue::BoundName(ref name) => {
                write!(f, "{}", name.value)?;
            }
            ExprValue::MemberAccess(value, ref member) => {
                write!(f, "{}.{}", value, member.value)?;
            }
            ExprValue::Call(ref name, ref args) => {
                write!(
                    f,
                    "{}({})",
                    name.value,
                    args.iter()
                        .map(|arg| arg.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
            }
            ExprValue::ArrayLiteral(ref elements, ref should_continue) => {
                write!(
                    f,
                    "[{}{}]",
                    elements
                        .iter()
                        .map(|elem| elem.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    if should_continue.is_some() {
                        format!(
                            "{}...",
                            if elements.is_empty() { "" } else { ", " }
                        )
                    } else {
                        "".into()
                    }
                )?;
            }
            ExprValue::PrefixOp(ref op, rhs) => {
                write!(f, "({} {})", op.value, rhs)?;
            }
            ExprValue::InfixBop(lhs, ref op, rhs) => {
                write!(f, "({} {} {})", lhs, op.value, rhs)?;
            }
            ExprValue::PostfixBop(lhs, ref op1, rhs, ref op2) => {
                write!(f, "({}{}{}{})", lhs, op1.value, rhs, op2.value)?;
            }
        }
        Ok(())
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}
