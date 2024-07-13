// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

use super::{node::Node, pretty_print::PrettyPrint, ty::Type};
use crate::token::Token;
use inform::fmt::IndentFormatter;
use pulsar_utils::pool::Handle;
use std::fmt::{Display, Write};

#[derive(Clone)]
pub enum ExprValue {
    ConstantInt(i64),
    /// TODO: Support `::`s
    BoundName(Token),

    MemberAccess(Handle<Expr>, Token),

    /// TODO: Call an `expr` or some sort of chaining of `::`
    Call(Token, Vec<Handle<Expr>>),

    /// `ArrayLiteral(elements, should_continue)` is an array literal beginning
    /// with `elements` and filling the remainder of the array with zeros if
    /// `should_continue`.
    ArrayLiteral(Vec<Handle<Expr>>, bool),

    PrefixOp(Token, Handle<Expr>),
    InfixBop(Handle<Expr>, Token, Handle<Expr>),
    PostfixBop(Handle<Expr>, Token, Handle<Expr>, Token),

    /// `HardwareMap(map_token, parallel_factor, f, arr)` is an array produced
    /// by applying `f` elementwise to `arr` using a hardware parallelism
    /// factor of `parallel_factor`.
    HardwareMap(Token, usize, Token, Handle<Expr>)
}

pub type Expr = Node<ExprValue, Handle<Type>>;

impl PrettyPrint for Expr {
    fn pretty_print(
        &self, f: &mut IndentFormatter<'_, '_>
    ) -> core::fmt::Result {
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
            ExprValue::ArrayLiteral(ref elements, should_continue) => {
                write!(
                    f,
                    "[{}{}]",
                    elements
                        .iter()
                        .map(|elem| elem.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    if should_continue {
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
            ExprValue::HardwareMap(_, ref parallel_factor, ref fun, arr) => {
                write!(f, "map<{}>({}, {})", parallel_factor, fun.value, arr)?;
            }
        }

        // let expr_ty = self.ty.as_ref();
        // if expr_ty.clone().is_known() {
        //     write!(f, ": {}", expr_ty)?;
        // }

        Ok(())
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}
