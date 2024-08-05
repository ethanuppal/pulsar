//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use decl::Decl;
use expr::Expr;
use node::AsNodePool;
use pulsar_utils::pool::HandleArray;
use stmt::Stmt;
use ty::AsTypePool;

pub mod decl;
pub mod expr;
pub mod node;
pub mod pretty_print;
pub mod stmt;
pub mod ty;

pub type AST = HandleArray<Decl>;

pub trait AsASTPool:
    AsTypePool + AsNodePool<Expr> + AsNodePool<Decl> + AsNodePool<Stmt> {
}
