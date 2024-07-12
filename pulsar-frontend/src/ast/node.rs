// Copyright (C) 2024 Ethan Uppal. This program is free software: you can
// redistribute it and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.

use crate::{
    attribute::{Attribute, Attributes},
    token::{Token, TokenType}
};
use pulsar_utils::loc::{Loc, SpanProvider};
use std::{
    fmt::{self, Display},
    hash::Hash,
    marker::PhantomData
};

pub trait NodeInterface: Sized + SpanProvider {
    type V;
    type T;

    /// Constructs a new AST node with the given `value` that ranges from
    /// `start_token` to `end_token`.
    fn new(value: Self::V, start_token: Token, end_token: Token) -> Self {
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
        value: Self::V, start_token: Token, end_token: Token,
        attributes: Attributes
    ) -> Self;

    /// The first token in this node.
    fn start_token(&self) -> &Token;

    /// The last token in this node.
    fn end_token(&self) -> &Token;

    /// Whether this node has the attribute `attr`.
    fn has_attribute(&self, attr: Attribute) -> bool;

    /// Annotates this node with the attribute `attr`.
    fn add_attribute(&mut self, attr: Attribute);
}

/// AST node with attributes.
pub struct Node<V, T> {
    pub value: V,
    attributes: Attributes,
    start_token: Token,
    end_token: Token,
    metadata: PhantomData<T>
}

impl<V, T> NodeInterface for Node<V, T> {
    type V = V;
    type T = T;

    fn new_with_attributes(
        value: V, start_token: Token, end_token: Token, attributes: Attributes
    ) -> Self {
        Self {
            value,
            attributes,
            start_token,
            end_token,
            metadata: PhantomData::default()
        }
    }

    fn start_token(&self) -> &Token {
        &self.start_token
    }

    fn end_token(&self) -> &Token {
        &self.end_token
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
            start_token: self.start_token.clone(),
            end_token: self.end_token.clone(),
            metadata: PhantomData::default()
        }
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

/// Pointer to an AST node in a pool.
pub struct Handle<T: NodeInterface> {
    index: usize,
    generic: PhantomData<T>
}

impl<T: NodeInterface> From<usize> for Handle<T> {
    fn from(value: usize) -> Self {
        Self {
            index: value,
            generic: PhantomData::default()
        }
    }
}

impl<T: NodeInterface> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            index: self.index.clone(),
            generic: PhantomData::default()
        }
    }
}

impl<T: NodeInterface> Copy for Handle<T> {}

impl<T: NodeInterface> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<T: NodeInterface> Eq for Handle<T> {}

impl<T: NodeInterface> Hash for Handle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

/// Allocator for AST nodes. See the [`NodePool`] trait.
pub struct BaseNodePool<N: NodeInterface> {
    contents: Vec<N>,
    metadata: Vec<N::T>
}

impl<N: NodeInterface> Default for BaseNodePool<N> {
    fn default() -> Self {
        Self {
            contents: Vec::default(),
            metadata: Vec::default()
        }
    }
}

pub trait AsNodePool<N: NodeInterface>: Sized {
    fn base_ref(&self) -> &BaseNodePool<N>;
    fn base_mut(&mut self) -> &mut BaseNodePool<N>;

    fn new(
        &mut self, value: N::V, start_token: Token, end_token: Token
    ) -> Handle<N> {
        self.new_with_attributes(
            value,
            start_token,
            end_token,
            Attributes::default()
        )
    }

    fn new_with_attributes(
        &mut self, value: N::V, start_token: Token, end_token: Token,
        attributes: Attributes
    ) -> Handle<N> {
        let index = self.base_ref().contents.len();
        self.base_mut().contents.push(N::new_with_attributes(
            value,
            start_token,
            end_token,
            attributes
        ));
        unsafe {
            let new_length = self.base_ref().contents.len();
            self.base_mut().metadata.reserve(1);
            self.base_mut().metadata.set_len(new_length);
        }
        Handle::from(index)
    }

    fn get(&self, handle: Handle<N>) -> &N {
        &self.base_ref().contents[handle.index]
    }

    fn get_mut(&mut self, handle: Handle<N>) -> &mut N {
        &mut self.base_mut().contents[handle.index]
    }

    fn ty<'a, 'b: 'a>(&'b self, handle: Handle<N>) -> &'a N::T
    where
        N: 'a {
        &self.base_ref().metadata[handle.index]
    }

    fn set_ty(&mut self, handle: Handle<N>, ty: N::T) {
        self.base_mut().metadata[handle.index] = ty;
    }
}

impl<N: NodeInterface> AsNodePool<N> for BaseNodePool<N> {
    fn base_ref(&self) -> &BaseNodePool<N> {
        self
    }

    fn base_mut(&mut self) -> &mut BaseNodePool<N> {
        self
    }
}
