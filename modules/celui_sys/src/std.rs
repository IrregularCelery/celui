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

#[no_mangle]
unsafe extern "C" fn _alloc_raw(size: usize, align: usize) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align_unchecked(size, align);

    std::alloc::alloc(layout)
}

#[no_mangle]
unsafe extern "C" fn _dealloc_raw(ptr: *mut u8, size: usize, align: usize) {
    if ptr.is_null() {
        return;
    }

    let layout = std::alloc::Layout::from_size_align_unchecked(size, align);

    std::alloc::dealloc(ptr, layout);
}
