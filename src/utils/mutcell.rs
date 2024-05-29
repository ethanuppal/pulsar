// Copyright (C) 2024 Ethan Uppal. All rights reserved.
use std::{
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard}
};

/// A mutable shared pointer.
pub struct MutCell<T> {
    pointer: Arc<RwLock<T>>
}

impl<T> MutCell<T> {
    pub fn new(value: T) -> Self {
        MutCell {
            pointer: Arc::new(RwLock::new(value))
        }
    }

    pub fn as_ref(&self) -> RwLockReadGuard<T> {
        self.pointer.read().unwrap()
    }

    pub fn as_mut(&self) -> RwLockWriteGuard<T> {
        self.pointer.write().unwrap()
    }

    pub fn raw(&self) -> Arc<RwLock<T>> {
        self.pointer.clone()
    }
}

impl<T> Clone for MutCell<T> {
    fn clone(&self) -> Self {
        Self {
            pointer: self.pointer.clone()
        }
    }
}

impl<T: Clone> MutCell<T> {
    pub fn clone_out(&self) -> T {
        self.as_ref().clone()
    }
}

impl<T: PartialEq> PartialEq for MutCell<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.as_ref() == *other.as_ref()
    }
}

impl<T: Eq> Eq for MutCell<T> {}

impl<T: Hash> Hash for MutCell<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state)
    }
}

impl<T: Display> Display for MutCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl<T: Debug> Debug for MutCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}
