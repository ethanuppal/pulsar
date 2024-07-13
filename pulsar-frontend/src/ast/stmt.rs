// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

use super::{
    expr::Expr, node::Node, pretty_print::PrettyPrint, stmt_ty::StmtType,
    ty::Type
};
use crate::token::Token;
use pulsar_utils::pool::Handle;
use std::fmt::{Display, Write};

pub type Param = (Token, Handle<Type>);

#[derive(Clone)]
pub enum StmtValue {
    Function {
        name: Token,
        params: Vec<Param>,
        open_paren: Token,
        ret: Handle<Type>,
        close_paren: Token,
        body: Vec<Handle<Stmt>>
    },
    LetBinding {
        name: Token,
        hint: Option<Handle<Type>>,
        value: Handle<Expr>
    },
    Return {
        ret_token: Token,
        value: Option<Handle<Expr>>
    }
}

pub type Stmt = Node<StmtValue, StmtType>;

impl PrettyPrint for Stmt {
    fn pretty_print(
        &self, f: &mut inform::fmt::IndentFormatter<'_, '_>
    ) -> core::fmt::Result {
        match self.value {
            StmtValue::Function {
                ref name,
                open_paren: _,
                ref params,
                close_paren: _,
                ret,
                ref body
            } => {
                let insert_newline = if body.is_empty() { "" } else { "\n" };
                writeln!(
                    f,
                    "func {}({}) -> {} {{{}",
                    name.value,
                    params
                        .iter()
                        .map(|(name, ty)| format!("{}: {}", name.value, ty))
                        .collect::<Vec<_>>()
                        .join(", "),
                    ret,
                    insert_newline,
                )?;
                f.increase_indent();
                for node in body {
                    node.pretty_print(f)?;
                    writeln!(f)?
                }
                f.decrease_indent();
                write!(f, "{}}}", insert_newline)?;
            }
            StmtValue::LetBinding {
                ref name,
                hint: hint_opt,
                value
            } => {
                let hint_str = if let Some(hint) = hint_opt {
                    format!(": {}", hint)
                } else {
                    "".into()
                };
                write!(f, "let {}{} = {}", name.value, hint_str, value)?;
            }
            StmtValue::Return {
                ret_token: _,
                value: value_opt
            } => {
                write!(f, "return")?;
                if let Some(value) = value_opt {
                    write!(f, " {}", value)?;
                }
            }
        }
        Ok(())
    }
}

impl Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}
