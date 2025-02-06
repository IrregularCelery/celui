// -------------------------------- std.rs --------------------------------- //

//! Support functions and utility methods for a standard (`std`) environment.
//!
//! This file is included in the build *only* when the `std` feature is enabled.
//! If the `std` feature is *not* enabled, this file will be excluded from compilation.
//!
//! To enable standard library support, add the `std` feature to your `Cargo.toml`:
//!
//! ```toml
//! [features]
//! std = []
//! ```

// ------------------------------- alloc.rs -------------------------------- //

pub mod alloc {

    #[inline(always)]
    pub fn alloc<T>() -> *mut T {
        let size = core::mem::size_of::<T>();
        let align = core::mem::align_of::<T>();

        unsafe {
            let layout = std::alloc::Layout::from_size_align_unchecked(size, align);

            std::alloc::alloc(layout) as *mut T
        }
    }

    #[inline(always)]
    pub fn dealloc<T>(ptr: *mut T) {
        if ptr.is_null() {
            return;
        }

        let size = core::mem::size_of::<T>();
        let align = core::mem::align_of::<T>();

        unsafe {
            let layout = std::alloc::Layout::from_size_align_unchecked(size, align);

            std::alloc::dealloc(ptr as *mut u8, layout);
        }
    }
}
