//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use memmap2::MmapMut;
use std::{
    fmt::{Debug, Display},
    hash::Hash,
    io,
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut}
};

/// 64MB.
const ARENA_SIZE_BYTES: usize = 64 * 1024 * 1024;

/// Pointer to a value allocated in a [`Pool`].
pub struct Handle<T> {
    pointer: *mut T
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.pointer }
    }
}

impl<T> DerefMut for Handle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.pointer }
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Handle<T> {}

impl<T: PartialEq> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { (*self.pointer).eq(&*other.pointer) }
    }
}

impl<T: Eq> Eq for Handle<T> {}

impl<T> Hash for Handle<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pointer.hash(state);
    }
}

impl<T: Display> Display for Handle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pointer.fmt(f)
    }
}

impl<T> From<*mut T> for Handle<T> {
    fn from(value: *mut T) -> Self {
        Self { pointer: value }
    }
}

struct MMapArena<T> {
    start: MmapMut,
    offset: usize,
    generic: PhantomData<T>
}

impl<T> MMapArena<T> {
    fn new(size: usize) -> io::Result<Self> {
        Ok(Self {
            start: MmapMut::map_anon(size)?,
            offset: 0,
            generic: PhantomData
        })
    }

    unsafe fn alloc(&mut self) -> *mut T {
        if self.offset + mem::size_of::<T>() > self.start.len() {
            panic!("Arena memory exhausted");
        }
        let result = self.start.as_mut_ptr().add(self.offset);
        self.offset += mem::size_of::<T>();
        result as *mut T
    }

    unsafe fn offset_of(&self, pointer: *mut T) -> usize {
        (pointer as *const u8).offset_from(self.start.as_ptr()) as usize
    }

    unsafe fn at_offset_ref(&self, offset: usize) -> *const T {
        self.start.as_ptr().add(offset) as *mut T
    }

    unsafe fn at_offset_mut(&mut self, offset: usize) -> *mut T {
        self.start.as_mut_ptr().add(offset) as *mut T
    }
}

/// Memory pool. See the [`AsPool`] trait.
pub struct Pool<Value, Metadata> {
    contents: MMapArena<Value>,
    metadata: MMapArena<Metadata>
}

impl<Value, Metadata> Pool<Value, Metadata> {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            contents: MMapArena::new(ARENA_SIZE_BYTES)?,
            metadata: MMapArena::new(ARENA_SIZE_BYTES)?
        })
    }

    /// Adds `value` to the pool at the returned handle with uninitialized
    /// metadata.`
    fn add(&mut self, value: Value) -> Handle<Value> {
        unsafe {
            let next_value = self.contents.alloc();
            *next_value = value;
            self.metadata.alloc();
            Handle::from(next_value)
        }
    }

    /// Requires: [`AsPool::set_metadata`] has been called at least once.
    fn get_metadata<'a, 'b: 'a>(
        &'b self, handle: Handle<Value>
    ) -> &'a Metadata
    where
        Value: 'a {
        unsafe {
            let offset = self.contents.offset_of(handle.pointer);
            self.metadata.at_offset_ref(offset).as_ref().unwrap()
        }
    }

    fn set_metadata(&mut self, handle: Handle<Value>, ty: Metadata) {
        unsafe {
            let offset = self.contents.offset_of(handle.pointer);
            *self.metadata.at_offset_mut(offset) = ty;
        }
    }
}

/// Can be used as a memory pool for pairing `Value`s with `Metadata`. See
/// [`Pool`].
pub trait AsPool<Value, Metadata>: Sized {
    fn base_ref(&self) -> &Pool<Value, Metadata>;
    fn base_mut(&mut self) -> &mut Pool<Value, Metadata>;

    /// Adds `value` to the pool at the returned handle with uninitialized
    /// metadata.`
    fn add(&mut self, value: Value) -> Handle<Value> {
        self.base_mut().add(value)
    }

    fn duplicate(&mut self, handle: Handle<Value>) -> Handle<Value>
    where
        Value: Clone,
        Metadata: Clone {
        let value_copy = (*handle).clone();
        let metadata_copy = self.get_metadata(handle).clone();
        let result = self.add(value_copy);
        self.set_metadata(result, metadata_copy);
        result
    }

    /// Requires: [`AsPool::set_metadata`] has been called at least once.
    fn get_metadata<'a, 'b: 'a>(
        &'b self, handle: Handle<Value>
    ) -> &'a Metadata
    where
        Value: 'a {
        self.base_ref().get_metadata(handle)
    }

    fn set_metadata(&mut self, handle: Handle<Value>, ty: Metadata) {
        self.base_mut().set_metadata(handle, ty);
    }
}

impl<Value, Metadata> AsPool<Value, Metadata> for Pool<Value, Metadata> {
    fn base_ref(&self) -> &Pool<Value, Metadata> {
        self
    }

    fn base_mut(&mut self) -> &mut Pool<Value, Metadata> {
        self
    }
}
