// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

use super::{
    node::{AsNodePool, Handle, NodeInterface},
    AsASTPool
};
use core::fmt;
use inform::fmt::IndentFormatter;
use pulsar_utils::format::INDENT_WIDTH;
use std::fmt::Display;

/// An AST node that can be pretty-printed.
pub trait PrettyPrint {
    /// Formats the current node using the given formatter `f` and the AST
    /// context `ast`.
    fn fmt<P: AsASTPool>(
        &self, f: &mut IndentFormatter<'_, '_>, ast_pool: &P
    ) -> fmt::Result;
}

pub struct NodeDisplayer<'ast, P: AsASTPool, N: NodeInterface>
where
    P: AsNodePool<N> {
    ast_pool: &'ast P,
    handle: Handle<N>
}

impl<'ast, P: AsASTPool, N: NodeInterface> Display for NodeDisplayer<'ast, P, N>
where
    P: AsNodePool<N>,
    N: PrettyPrint
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = IndentFormatter::new(f, INDENT_WIDTH);
        self.ast_pool.get(self.handle).fmt(&mut f, self.ast_pool)
    }
}

pub trait PrettyPrinter<N: NodeInterface + PrettyPrint>
where
    Self: AsASTPool + AsNodePool<N> {
    /// Formats a `handle` belonging to this pool with the given formatter `f`.
    fn fmt(
        &self, f: &mut IndentFormatter<'_, '_>, handle: Handle<N>
    ) -> fmt::Result;

    /// Constructs a type implementing [`Display`] for the a `handle` belonging
    /// to this pool for directly passing to [`write!`] or [`format!`].
    fn fmtr(&self, handle: Handle<N>) -> NodeDisplayer<Self, N>;

    /// Constructs a string representation of the `handle` belonging to this
    /// pool.
    fn to_string(&self, handle: Handle<N>) -> String {
        self.fmtr(handle).to_string()
    }
}

impl<N: NodeInterface + PrettyPrint, P: AsASTPool> PrettyPrinter<N> for P
where
    P: AsNodePool<N>
{
    fn fmt(
        &self, f: &mut IndentFormatter<'_, '_>, handle: Handle<N>
    ) -> fmt::Result {
        self.get(handle).fmt(f, self)
    }

    fn fmtr(&self, handle: Handle<N>) -> NodeDisplayer<Self, N> {
        NodeDisplayer {
            ast_pool: self,
            handle
        }
    }
}
