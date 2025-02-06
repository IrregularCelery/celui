// ------------------------------- alloc.rs -------------------------------- //

//! Memory allocation interface.
//!
//! This module provides an interface for memory allocation, defining functions
//! for allocating and deallocating memory. When the `std` feature is enabled,
//! these functions use the standard library's allocation mechanisms. Otherwise,
//! they provide a set of extern "C" functions that must be implemented by the user.

// ------------------------------- Allocate ------------------------------- //

#[inline(always)]
pub fn alloc<T>() -> *mut T {
    let size = core::mem::size_of::<T>();
    let align = core::mem::align_of::<T>();

    unsafe { _alloc_raw(size, align) as *mut T }
}

#[inline(always)]
pub fn alloc_many<T>(count: usize) -> *mut T {
    if count < 1 {
        return core::ptr::null_mut();
    }

    let size = core::mem::size_of::<T>() * count;
    let align = core::mem::align_of::<T>();

    unsafe { _alloc_raw(size, align) as *mut T }
}

// ------------------------------ Deallocate ------------------------------- //

#[inline(always)]
pub fn dealloc<T>(ptr: *mut T) {
    if ptr.is_null() {
        return;
    }

    let size = core::mem::size_of::<T>();
    let align = core::mem::align_of::<T>();

    unsafe { _dealloc_raw(ptr as *mut u8, size, align) };
}

#[inline(always)]
pub fn dealloc_many<T>(ptr: *mut T, count: usize) {
    if ptr.is_null() || count < 1 {
        return;
    }

    let size = core::mem::size_of::<T>() * count;
    let align = core::mem::align_of::<T>();

    unsafe { _dealloc_raw(ptr as *mut u8, size, align) };
}

// ------------------------------ Interfaces ------------------------------- //

extern "C" {
    fn _alloc_raw(size: usize, align: usize) -> *mut u8;
    fn _dealloc_raw(ptr: *mut u8, size: usize, align: usize) -> *mut u8;
}
