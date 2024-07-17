//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use crate::{
    attribute::{Attribute, Attributes},
    token::{Token, TokenType}
};
use pulsar_utils::{
    loc::{Loc, SpanProvider},
    pool::{AsPool, Handle}
};
use std::{
    fmt::{self, Display},
    marker::PhantomData
};

pub trait NodeInterface: Sized + SpanProvider {
    type V;
    type T;

    /// Constructs a new AST node with the given `value` that ranges from
    /// `start_token` to `end_token`.
    fn new(
        value: Self::V, start_token: Handle<Token>, end_token: Handle<Token>
    ) -> Self {
        Self::new_with_attributes(
            value,
            start_token,
            end_token,
            Attributes::default()
        )
    }

    /// Constructs a new AST node with the given `value` and `attributes` that
    /// ranges from `start_token` to `end_token`.
    fn new_with_attributes(
        value: Self::V, start_token: Handle<Token>, end_token: Handle<Token>,
        attributes: Attributes
    ) -> Self;

    /// The first token in this node.
    fn start_token(&self) -> Handle<Token>;

    /// The last token in this node.
    fn end_token(&self) -> Handle<Token>;

    /// Whether this node has the attribute `attr`.
    fn has_attribute(&self, attr: Attribute) -> bool;

    /// Annotates this node with the attribute `attr`.
    fn add_attribute(&mut self, attr: Attribute);
}

/// AST node with attributes.
pub struct Node<V, T> {
    pub value: V,
    attributes: Attributes,
    start_token: Handle<Token>,
    end_token: Handle<Token>,
    metadata: PhantomData<T>
}

impl<V, T> NodeInterface for Node<V, T> {
    type V = V;
    type T = T;

    fn new_with_attributes(
        value: V, start_token: Handle<Token>, end_token: Handle<Token>,
        attributes: Attributes
    ) -> Self {
        Self {
            value,
            attributes,
            start_token,
            end_token,
            metadata: PhantomData
        }
    }

    fn start_token(&self) -> Handle<Token> {
        self.start_token
    }

    fn end_token(&self) -> Handle<Token> {
        self.end_token
    }

    fn has_attribute(&self, attr: Attribute) -> bool {
        self.attributes.has(attr)
    }

    fn add_attribute(&mut self, attr: Attribute) {
        self.attributes.add(attr);
    }
}

impl<V: Clone, T> Clone for Node<V, T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            attributes: self.attributes,
            start_token: self.start_token,
            end_token: self.end_token,
            metadata: PhantomData
        }
    }
}

impl<V, T> AsRef<Node<V, T>> for Node<V, T> {
    fn as_ref(&self) -> &Node<V, T> {
        self
    }
}

impl<V, T> SpanProvider for Node<V, T> {
    fn start(&self) -> Loc {
        self.start_token.loc.clone()
    }

    fn end(&self) -> Loc {
        let end_token = &self.end_token;
        let mut loc = end_token.loc.clone();
        // tokens are always on one line
        if end_token.ty != TokenType::Newline {
            let length = end_token.value.len() as isize;
            loc.pos += length;
            loc.col += length;
        }
        loc
    }
}

impl<V: Display, T> Display for Node<V, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

/// Can be used as a [`NodePool`].
pub trait AsNodePool<N: NodeInterface>: AsPool<N, N::T> {
    #[allow(clippy::new_ret_no_self)]
    #[allow(clippy::wrong_self_convention)]
    fn new(
        &mut self, value: N::V, start_token: Handle<Token>,
        end_token: Handle<Token>
    ) -> Handle<N> {
        self.new_with_attributes(
            value,
            start_token,
            end_token,
            Attributes::default()
        )
    }

    fn new_with_attributes(
        &mut self, value: N::V, start_token: Handle<Token>,
        end_token: Handle<Token>, attributes: Attributes
    ) -> Handle<N> {
        self.add(N::new_with_attributes(
            value,
            start_token,
            end_token,
            attributes
        ))
    }

    fn generate(
        &mut self, value: N::V, start_token: Handle<Token>,
        end_token: Handle<Token>
    ) -> Handle<N> {
        self.new_with_attributes(
            value,
            start_token,
            end_token,
            Attributes::from([Attribute::Generated])
        )
    }

    fn get_ty<'a, 'b: 'a>(&'b self, handle: Handle<N>) -> &'a N::T
    where
        N: 'a {
        self.get_metadata(handle)
    }

    fn set_ty(&mut self, handle: Handle<N>, ty: N::T) {
        self.set_metadata(handle, ty);
    }
}

// Memory pool for AST nodes.
// pub type NodePool<N: NodeInterface> = Pool<N::V, N::T>;
