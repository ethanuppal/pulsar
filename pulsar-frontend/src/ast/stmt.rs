// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

use super::{
    expr::Expr,
    node::{Handle, Node},
    pretty_print::{PrettyPrint, PrettyPrinter},
    stmt_ty::StmtType,
    ty::Type,
    AsASTPool
};
use crate::token::Token;
use std::fmt::Write;

pub type Param = (Token, Handle<Type>);

#[derive(Clone)]
pub enum StmtValue {
    Function {
        name: Token,
        params: Vec<Param>,
        ret: Handle<Type>,
        pure_token: Option<Token>,
        body: Vec<Handle<Stmt>>
    },
    LetBinding {
        name: Token,
        hint: Option<Handle<Type>>,
        value: Handle<Expr>
    },
    Return {
        keyword_token: Token,
        value: Option<Handle<Expr>>
    }
}

pub type Stmt = Node<StmtValue, StmtType>;

impl PrettyPrint for Stmt {
    fn fmt<P: AsASTPool>(
        &self, f: &mut inform::fmt::IndentFormatter<'_, '_>, ast_pool: &P
    ) -> core::fmt::Result {
        match self.value {
            StmtValue::Function {
                ref name,
                ref params,
                ret,
                ref pure_token,
                ref body
            } => {
                let insert_newline = if body.is_empty() { "" } else { "\n" };
                writeln!(
                    f,
                    "{}func {}({}) -> {} {{{}",
                    if pure_token.is_some() { "pure " } else { "" },
                    name.value,
                    params
                        .iter()
                        .map(|(name, ty)| format!(
                            "{}: {}",
                            name.value,
                            ast_pool.fmtr(*ty)
                        ))
                        .collect::<Vec<_>>()
                        .join(", "),
                    ast_pool.fmtr(ret),
                    insert_newline,
                )?;
                f.increase_indent();
                for node in body {
                    ast_pool.fmt(f, *node)?;
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
                    format!(": {}", ast_pool.fmtr(hint))
                } else {
                    "".into()
                };
                write!(
                    f,
                    "let {}{} = {}",
                    name.value,
                    hint_str,
                    ast_pool.fmtr(value)
                )?;
            }
            StmtValue::Return {
                keyword_token: _,
                value: value_opt
            } => {
                write!(f, "return")?;

                if let Some(value) = value_opt {
                    write!(f, " {}", ast_pool.fmtr(value))?;
                }
            }
        }
        Ok(())
    }
}
