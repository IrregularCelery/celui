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

// -------------------------------- Color ---------------------------------- //

impl std::fmt::Debug for crate::Color {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

// -------------------------------- Matrix --------------------------------- //

impl std::fmt::Debug for crate::Mat2 {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mat2({:?})", self.elements)
    }
}

impl std::fmt::Debug for crate::Mat3 {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mat3({:?})", self.elements)
    }
}

impl std::fmt::Debug for crate::Mat4 {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mat4({:?})", self.elements)
    }
}

// --------------------------------- Rect ---------------------------------- //

impl std::fmt::Debug for crate::Rect {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rect({}, {}, {}, {})",
            self.x, self.y, self.width, self.height
        )
    }
}

// -------------------------------- Vector --------------------------------- //

impl std::fmt::Debug for crate::Vec2 {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2({}, {})", self.x, self.y)
    }
}

impl std::fmt::Debug for crate::Vec3 {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl std::fmt::Debug for crate::Vec4 {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec4({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}
