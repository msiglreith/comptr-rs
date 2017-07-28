
extern crate winapi;

use std::fmt;
use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};

use winapi::IUnknown;

#[derive(PartialEq, Eq)]
pub struct ComPtr<T> {
    pointer: *mut T,
}

impl<T> ComPtr<T> {
    #[inline]
    pub fn new(ptr: *mut T) -> ComPtr<T> {
        ComPtr { pointer: ptr }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.pointer
    }

    #[inline]
    pub unsafe fn as_mut_ptr(&self) -> *mut T {
        self.pointer
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        self.pointer.is_null()
    }
}

impl<T> fmt::Pointer for ComPtr<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&self.pointer, f)
    }
}

impl<T> fmt::Debug for ComPtr<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(self, f)
    }
}

impl<T> AsRef<*mut T> for ComPtr<T> {
    #[inline]
    fn as_ref(&self) -> & *mut T {
        &self.pointer
    }
}

impl<T> AsMut<*mut T> for ComPtr<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut *mut T {
        &mut self.pointer
    }
}

impl<T> Deref for ComPtr<T> {
    type Target = T;

    #[inline]
    fn deref<'a>(&'a self) -> &'a T {
        assert!(!self.is_null(), "can't deref null-ptr!");
        unsafe { &*self.pointer }
    }
}

impl<T> DerefMut for ComPtr<T> {
    #[inline]
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        assert!(!self.is_null(), "can't deref null-ptr!");
        unsafe { &mut *self.pointer }
    }
}

impl<T> Clone for ComPtr<T> {
    #[inline]
    fn clone(&self) -> ComPtr<T> {
        if !self.is_null() {
            unsafe { (*(self.pointer as *mut IUnknown)).AddRef(); }
        }

        ComPtr { pointer: self.pointer }
    }
}

impl<T> Drop for ComPtr<T> {
    #[inline]
    fn drop(&mut self) {
        if !self.is_null() {
            unsafe { (*(self.pointer as *mut IUnknown)).Release(); }
        }
    }
}

impl<T> Hash for ComPtr<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pointer.hash(state)
    }
}
