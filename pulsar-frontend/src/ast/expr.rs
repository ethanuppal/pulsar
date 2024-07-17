//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::{node::Node, pretty_print::PrettyPrint, ty::Type};
use crate::token::Token;
use inform::fmt::IndentFormatter;
use pulsar_utils::pool::Handle;
use std::fmt::{self, Display, Write};

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

pub type Expr = Node<ExprValue, Handle<Type>>;

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
