// ------------------------------- alloc.rs -------------------------------- //

//! Memory allocation interface.
//!
//! This module provides an interface for memory allocation, defining functions
//! for allocating and deallocating memory. When the `std` feature is enabled,
//! these functions use the standard library's allocation mechanisms. Otherwise,
//! they provide a set of extern "C" functions that must be implemented by the user.

// ------------------------------- Allocate ------------------------------- //

/// Allocates memory for a single item of type T.
///
/// # Safety
/// - The caller must ensure the allocated memory is properly initialized before use
/// - The caller is responsible for deallocating the memory using `dealloc`
///
/// # Returns
/// - Returns null if T is zero-sized
/// - Returns null on allocation failure
/// - Otherwise returns a aligned pointer to allocated memory
#[inline(always)]
pub unsafe fn alloc<T>() -> *mut T {
    // Handle zero-sized types (ZSTs)
    if core::mem::size_of::<T>() == 0 {
        return core::ptr::NonNull::dangling().as_ptr();
    }

    let size = core::mem::size_of::<T>();
    let align = core::mem::align_of::<T>();

    unsafe { _alloc_raw(size, align) as *mut T }
}

/// Allocates memory for `count` items of type T.
///
/// # Safety
/// - The caller must ensure the allocated memory is properly initialized before use
/// - The caller is responsible for deallocating the memory using `dealloc_many`
///
/// # Returns
/// - Returns null if count is 0 or T is zero-sized
/// - Returns null on allocation failure or size overflow
/// - Otherwise returns a aligned pointer to allocated memory
#[inline(always)]
pub unsafe fn alloc_many<T>(count: usize) -> *mut T {
    // Early return for zero count or zero-sized types (ZSTs)
    if count == 0 || core::mem::size_of::<T>() == 0 {
        return core::ptr::NonNull::dangling().as_ptr();
    }

    // Check for size overflow
    let size = match core::mem::size_of::<T>().checked_mul(count) {
        Some(size) => size,
        None => return core::ptr::null_mut(),
    };
    let align = core::mem::align_of::<T>();

    unsafe { _alloc_raw(size, align) as *mut T }
}

// ------------------------------ Deallocate ------------------------------- //

/// Deallocates memory previously allocated for a single item of type T.
///
/// # Safety
/// - `ptr` must have been allocated using `alloc<T>`
/// - The memory must not be accessed after this call
/// - This function must not be called twice with the same pointer
#[inline(always)]
pub unsafe fn dealloc<T>(ptr: *mut T) {
    // Handle null pointers and zero-sized types (ZSTs)
    if ptr.is_null() || core::mem::size_of::<T>() == 0 {
        return;
    }

    let size = core::mem::size_of::<T>();
    let align = core::mem::align_of::<T>();

    unsafe { _dealloc_raw(ptr as *mut u8, size, align) };
}

/// Deallocates memory previously allocated for `count` items of type T.
///
/// # Safety
/// - `ptr` must have been allocated using `alloc_many<T>` with the same count
/// - The memory must not be accessed after this call
/// - This function must not be called twice with the same pointer
#[inline(always)]
pub unsafe fn dealloc_many<T>(ptr: *mut T, count: usize) {
    // Handle null pointers, zero count, and zero-sized types(ZSTs)
    if ptr.is_null() || count == 0 || core::mem::size_of::<T>() == 0 {
        return;
    }

    // Check for size overflow
    let size = match core::mem::size_of::<T>().checked_mul(count) {
        Some(size) => size,
        None => return,
    };
    let align = core::mem::align_of::<T>();

    unsafe { _dealloc_raw(ptr as *mut u8, size, align) };
}

// ------------------------------ Interfaces ------------------------------- //

extern "C" {
    fn _alloc_raw(size: usize, align: usize) -> *mut u8;
    fn _dealloc_raw(ptr: *mut u8, size: usize, align: usize) -> *mut u8;
}
