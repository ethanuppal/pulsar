//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use super::{expr::Expr, node::Node, pretty_print::PrettyPrint, ty::Type};
use crate::token::Token;
use inform::fmt::IndentFormatter;
use pulsar_utils::pool::Handle;
use std::fmt::{self, Display, Write};

#[derive(Clone)]
pub enum StmtValue {
    Let {
        name: Handle<Token>,
        hint: Option<Handle<Type>>,
        value: Handle<Expr>
    },
    Assign(Handle<Expr>, Handle<Token>, Handle<Expr>),
    Divider(Handle<Token>)
}

pub type Stmt = Node<StmtValue, ()>;

impl PrettyPrint for Stmt {
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result {
        match self.value {
            StmtValue::Let {
                name,
                hint: hint_opt,
                value
            } => {
                let hint_str = if let Some(hint) = hint_opt {
                    format!(": {}", hint)
                } else {
                    "".into()
                };
                write!(f, "let {}{} = {}", name.value, hint_str, value)
            }
            StmtValue::Assign(lhs, equals, rhs) => {
                write!(f, "{} {} {}", lhs, equals.value, rhs)
            }
            StmtValue::Divider(_) => write!(f, "---")
        }
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}
