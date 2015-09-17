
extern crate winapi;

use std::fmt;
use std::ops::{Deref, DerefMut};

use winapi::IUnknown;

// TODO: NonZero?
pub struct ComPtr<T> {
    pointer: *mut T,
}

impl<T> ComPtr<T> {
    pub fn new(ptr: *mut T) -> ComPtr<T> {
        ComPtr { pointer: ptr }
    }

    pub fn as_ptr(&self) -> *const T {
        self.pointer
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        self.pointer
    }

    pub fn get_ref(&mut self) -> & *mut T {
        &self.pointer
    }

    pub fn get_mut_ref(&mut self) -> &mut *mut T {
        &mut self.pointer
    }

    pub fn is_null(&self) -> bool {
        self.pointer.is_null()
    }
}

impl<T> fmt::Pointer for ComPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&self.pointer, f)
    }
}

impl<T> fmt::Debug for ComPtr<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(self, f)
    }
}

impl<T> Deref for ComPtr<T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        assert!(!self.is_null(), "can't deref null-ptr!");
        unsafe { &*self.pointer }
    }
}

impl<T> DerefMut for ComPtr<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        assert!(!self.is_null(), "can't deref null-ptr!");
        unsafe { &mut *self.pointer }
    }
}

impl<T> Clone for ComPtr<T> {
    fn clone(&self) -> ComPtr<T> {
        if !self.is_null() {
            unsafe { (*(self.pointer as *mut IUnknown)).AddRef(); }
        }

        ComPtr { pointer: self.pointer }
    }
}

impl<T> Drop for ComPtr<T> {
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { (*(self.pointer as *mut IUnknown)).Release(); }
        }
    }
}

// TODO: Send, Sync, From, Into
