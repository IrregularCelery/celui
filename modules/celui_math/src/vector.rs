// ------------------------------ vector.rs -------------------------------- //

//! N-dimensional vector representation and operations.
//!
//! This module provides a `Vector` struct for mathematical operations.
//! It supports creation, access, modification, and common vector arithmetic
//! like addition, subtraction, scalar multiplication, dot product, and normalization.

// --------------------------------- Vec2 ---------------------------------- //

#[derive(Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    #[inline(always)]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl core::ops::Add for Vec2 {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl core::ops::Sub for Vec2 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl core::ops::Mul<f32> for Vec2 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl core::ops::Div<f32> for Vec2 {
    type Output = Self;

    #[inline(always)]
    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

// --------------------------------- Vec3 ---------------------------------- //

#[derive(Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl core::ops::Add for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl core::ops::Sub for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z + other.z,
        }
    }
}

impl core::ops::Mul<f32> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl core::ops::Div<f32> for Vec3 {
    type Output = Self;

    #[inline(always)]
    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

// --------------------------------- Vec4 ---------------------------------- //

#[derive(Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    #[inline(always)]
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl core::ops::Add for Vec4 {
    type Output = Self;

    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl core::ops::Sub for Vec4 {
    type Output = Self;

    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl core::ops::Mul<f32> for Vec4 {
    type Output = Self;

    #[inline(always)]
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl core::ops::Div<f32> for Vec4 {
    type Output = Self;

    #[inline(always)]
    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}
