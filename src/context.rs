//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use pulsar_frontend::{
    ast::{
        decl::Decl,
        expr::Expr,
        node::AsNodePool,
        stmt::Stmt,
        ty::{AsTypePool, LiquidType, Type},
        AsASTPool
    },
    token::Token,
    type_inferer::{AsInferencePool, LiquidTypeConstraint, TypeConstraint}
};
use pulsar_ir::{
    cell::Cell,
    control::{AsControlPool, Control},
    from_ast::AsGeneratorPool,
    port::Port
};
use pulsar_utils::{
    id::Gen,
    pool::{AsPool, Handle, Pool}
};
use std::io;

/// Stores all objects allocated in memory pools by the compiler.
pub struct Context {
    types: Pool<Type, ()>,
    liquid_types: Pool<LiquidType, ()>,
    exprs: Pool<Expr, Handle<Type>>,
    stmts: Pool<Stmt, ()>,
    decls: Pool<Decl, ()>,
    tokens: Pool<Token, ()>,
    type_constraints: Pool<TypeConstraint, ()>,
    liquid_type_constraints: Pool<LiquidTypeConstraint, ()>,
    controls: Pool<Control, usize>,
    cells: Pool<Cell, ()>,
    ports: Pool<Port, ()>
}

impl Context {
    pub fn new() -> io::Result<Self> {
        Ok(Context {
            types: Pool::new()?,
            liquid_types: Pool::new()?,
            exprs: Pool::new()?,
            stmts: Pool::new()?,
            decls: Pool::new()?,
            tokens: Pool::new()?,
            type_constraints: Pool::new()?,
            liquid_type_constraints: Pool::new()?,
            controls: Pool::new()?,
            cells: Pool::new()?,
            ports: Pool::new()?
        })
    }
}

impl AsMut<Context> for Context {
    fn as_mut(&mut self) -> &mut Context {
        self
    }
}

macro_rules! as_pool {
    ($value:ty, $metadata:ty, $source:ident) => {
        impl AsPool<$value, $metadata> for Context {
            fn as_pool_ref(&self) -> &Pool<$value, $metadata> {
                &self.$source
            }
            fn as_pool_mut(&mut self) -> &mut Pool<$value, $metadata> {
                &mut self.$source
            }
        }
    };
}

macro_rules! as_node_pool {
    ($node:ty, $source:ident) => {
        as_pool!(
            $node,
            <$node as crate::pulsar_frontend::ast::node::NodeInterface>::T,
            $source
        );
        impl AsNodePool<$node> for Context {}
    };
}

as_node_pool!(Type, types);
as_node_pool!(LiquidType, liquid_types);
impl AsTypePool for Context {}
as_node_pool!(Expr, exprs);
as_node_pool!(Stmt, stmts);
as_node_pool!(Decl, decls);
impl AsASTPool for Context {}
as_pool!(Token, (), tokens);
as_pool!(TypeConstraint, (), type_constraints);
as_pool!(LiquidTypeConstraint, (), liquid_type_constraints);
impl AsInferencePool for Context {}
as_pool!(Control, usize, controls);
impl AsControlPool for Context {}
as_pool!(Cell, (), cells);
as_pool!(Port, (), ports);
impl AsGeneratorPool for Context {}
