//! Copyright (C) 2024 Ethan Uppal. This program is free software: you can
//! redistribute it and/or modify it under the terms of the GNU General Public
//! License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.

use memmap2::{MmapMut, MmapOptions};
use std::{
    fmt::{self, Debug, Display},
    hash::Hash,
    io,
    marker::PhantomData,
    mem::{self, ManuallyDrop},
    ops::{Deref, DerefMut},
    ptr,
    rc::Rc
};

use crate::loc::Loc;

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

impl<T: Sized> AsRef<T> for Handle<T>
where
    <Handle<T> as Deref>::Target: AsRef<T>
{
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T> DerefMut for Handle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.pointer }
    }
}

impl<T: Sized> AsMut<T> for Handle<T>
where
    <Handle<T> as Deref>::Target: AsMut<T>
{
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut()
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { &*self.pointer }.fmt(f)
    }
}

impl<T: Debug> Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { &*self.pointer }.fmt(f)
    }
}

impl<T> From<*mut T> for Handle<T> {
    fn from(value: *mut T) -> Self {
        Self { pointer: value }
    }
}

union MMapArenaStore {
    mmap: ManuallyDrop<MmapMut>,
    ignore: ()
}

impl MMapArenaStore {
    fn none() -> Self {
        Self { ignore: () }
    }

    fn from(mmap: MmapMut) -> Self {
        Self {
            mmap: ManuallyDrop::new(mmap)
        }
    }
}

struct MMapArena<T> {
    store: MMapArenaStore,
    offset: usize,
    generic: PhantomData<T>
}

impl<T> MMapArena<T> {
    fn new(size: usize) -> io::Result<Self> {
        Ok(Self {
            store: if mem::size_of::<T>() == 0 {
                MMapArenaStore::none()
            } else {
                MMapArenaStore::from(MmapOptions::new().len(size).map_anon()?)
            },
            offset: 0,
            generic: PhantomData
        })
    }

    unsafe fn alloc(&mut self) -> *mut T {
        if mem::size_of::<T>() == 0 {
            ptr::null_mut()
        } else {
            let mmap = &mut self.store.mmap;
            if self.offset + mem::size_of::<T>() > mmap.len() {
                panic!("Arena memory exhausted");
            }
            let result = mmap.as_mut_ptr().add(self.offset);
            self.offset += mem::size_of::<T>();
            result as *mut T
        }
    }

    unsafe fn offset_of(&self, pointer: *mut T) -> usize {
        (pointer as *const u8).offset_from(self.store.mmap.as_ptr()) as usize
    }

    unsafe fn at_offset_ref(&self, offset: usize) -> *const T {
        self.store.mmap.as_ptr().add(offset) as *mut T
    }

    unsafe fn at_offset_mut(&mut self, offset: usize) -> *mut T {
        let mmap = &mut self.store.mmap;
        mmap.as_mut_ptr().add(offset) as *mut T
    }

    fn as_slice(&mut self) -> (*mut T, usize) {
        unsafe {
            (
                self.store.mmap.as_ptr() as *mut T,
                self.offset / mem::size_of::<T>()
            )
        }
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
            *(next_value as *mut ManuallyDrop<Value>) =
                ManuallyDrop::new(value);
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
    fn as_pool_ref(&self) -> &Pool<Value, Metadata>;
    fn as_pool_mut(&mut self) -> &mut Pool<Value, Metadata>;

    /// Adds `value` to the pool at the returned handle with uninitialized
    /// metadata.`
    fn add(&mut self, value: Value) -> Handle<Value> {
        self.as_pool_mut().add(value)
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
        self.as_pool_ref().get_metadata(handle)
    }

    fn set_metadata(&mut self, handle: Handle<Value>, ty: Metadata) {
        self.as_pool_mut().set_metadata(handle, ty);
    }
}

impl<Value, Metadata> AsPool<Value, Metadata> for Pool<Value, Metadata> {
    fn as_pool_ref(&self) -> &Pool<Value, Metadata> {
        self
    }

    fn as_pool_mut(&mut self) -> &mut Pool<Value, Metadata> {
        self
    }
}

impl<Value, Metadata> Pool<Value, Metadata> {
    pub fn as_array(&mut self) -> HandleArray<Value> {
        let (start, length) = self.contents.as_slice();
        HandleArray { start, length }
    }
}

/// An immutable array of handles. Internally just a pointer to the start of the
/// memory pool it was created from via [`Pool::as_array`].
pub struct HandleArray<T> {
    start: *mut T,
    length: usize
}

impl<T> HandleArray<T> {
    pub fn at(&self, index: usize) -> Handle<T> {
        Handle::from(unsafe { self.start.add(index) })
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn last(&self) -> Option<Handle<T>> {
        if self.is_empty() {
            None
        } else {
            Some(self.at(self.len() - 1))
        }
    }
}

impl<T> Clone for HandleArray<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for HandleArray<T> {}

pub struct HandleArrayIterator<T> {
    array: HandleArray<T>,
    index: usize
}

impl<T> Iterator for HandleArrayIterator<T> {
    type Item = Handle<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.array.len() {
            None
        } else {
            Some(Handle::from(unsafe { self.array.start.add(self.index) }))
        }
    }
}

impl<T> ExactSizeIterator for HandleArrayIterator<T> {
    fn len(&self) -> usize {
        self.array.len()
    }
}

impl<T> IntoIterator for HandleArray<T> {
    type Item = Handle<T>;
    type IntoIter = HandleArrayIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        HandleArrayIterator {
            array: self,
            index: 0
        }
    }
}
