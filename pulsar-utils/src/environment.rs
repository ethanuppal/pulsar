//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use std::{collections::HashMap, hash::Hash};

/// A set of bindings between names and values.
pub type Scope<Name, T> = HashMap<Name, T>;

/// A scoped set of bindings between names and values.
pub struct Environment<Name: Eq + Hash, T> {
    scopes: Vec<Scope<Name, T>>
}

impl<Name: Eq + Hash, T> Environment<Name, T> {
    /// Constructs a new context with a base scope that further scopes can be
    /// pushed and popped on top of.
    pub fn new() -> Self {
        Environment {
            scopes: vec![Scope::new()]
        }
    }

    /// Creates a new scope.
    pub fn push(&mut self) {
        self.scopes.push(Scope::new());
    }

    /// Removes all bindings in the most recent scope, returning `true` unless
    /// there are no scopes beside the base scope.
    pub fn pop(&mut self) -> bool {
        if self.scopes.len() == 1 {
            false
        } else {
            self.scopes.pop();
            true
        }
    }

    /// Binds `name` to `value` in the top scope, returning
    /// `Some(previous_value)` if `previous_value` had previously been bound to
    /// `name`, or `None`.
    pub fn bind(&mut self, name: Name, value: T) -> Option<T> {
        self.scopes.last_mut().unwrap().insert(name, value)
    }

    /// Binds `name` to `value` in the base scope.
    ///
    /// @see [`Environment::bind`]
    pub fn bind_base(&mut self, name: Name, value: T) -> Option<T> {
        self.scopes.first_mut().unwrap().insert(name, value)
    }

    /// Finds the bound value for `name` in the highest scope possible.
    pub fn find(&self, name: Name) -> Option<&T> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(&name) {
                return Some(value);
            }
        }
        None
    }
}

impl<Name: Eq + Hash, T> Default for Environment<Name, T> {
    fn default() -> Self {
        Self::new()
    }
}
