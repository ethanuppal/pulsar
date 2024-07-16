// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

use super::{node::Node, pretty_print::PrettyPrint, stmt::Stmt, ty::Type};
use crate::token::Token;
use inform::fmt::IndentFormatter;
use pulsar_utils::pool::Handle;
use std::fmt::{self, Display, Write};

pub type Param = (Handle<Token>, Handle<Type>);
pub type ParamVec = Vec<Param>;

impl PrettyPrint for ParamVec {
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result {
        for (i, (param_name, param_type)) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", param_name.value, param_type)?;
        }
        Ok(())
    }
}

pub enum DeclValue {
    Function {
        func: Handle<Token>,
        name: Handle<Token>,
        inputs: ParamVec,
        outputs: ParamVec,
        body: Vec<Handle<Stmt>>
    }
}

pub type Decl = Node<DeclValue, ()>;

impl PrettyPrint for Decl {
    fn pretty_print(&self, f: &mut IndentFormatter<'_, '_>) -> fmt::Result {
        match self.value {
            DeclValue::Function {
                func: _,
                ref name,
                ref inputs,
                ref outputs,
                ref body
            } => {
                let insert_newline = if body.is_empty() { "" } else { "\n" };
                writeln!(
                    f,
                    "func {}({}) -> ({}) {{{}",
                    name.value,
                    inputs.to_string(),
                    outputs.to_string(),
                    insert_newline,
                )?;
                f.increase_indent();
                for node in body {
                    node.pretty_print(f)?;
                    writeln!(f)?
                }
                f.decrease_indent();
                write!(f, "{}}}", insert_newline)
            }
        }
    }
}

impl Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        PrettyPrint::fmt(self, f)
    }
}
