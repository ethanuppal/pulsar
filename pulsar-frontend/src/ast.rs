// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

use expr::Expr;
use node::AsNodePool;
use stmt::Stmt;
use ty::AsTypePool;

pub mod expr;
pub mod node;
pub mod pretty_print;
pub mod stmt;
pub mod stmt_ty;
pub mod ty;

pub trait AsASTPool: AsTypePool + AsNodePool<Expr> + AsNodePool<Stmt> {}

// impl ASTPool {
//     pub fn new() -> Self {
//         Self::default()
//     }
// }

// impl AsRef<ASTPool> for ASTPool {
//     fn as_ref(&self) -> &ASTPool {
//         &self
//     }
// }
