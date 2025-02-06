// ------------------------------- color.rs -------------------------------- //

//! Color representation and manipulation.
//!
//! This module defines structures and functions for working with colors,
//! including different color formats (e.g., RGB, HSL, HEX) and related operations.

// -------------------------------- Color ---------------------------------- //

#[derive(Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    #[inline(always)]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    #[inline(always)]
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }
}

impl Color {
    pub const BLACK: Self = Self::from_rgb(0, 0, 0);
    pub const WHITE: Self = Self::from_rgb(255, 255, 255);
    pub const RED: Self = Self::from_rgb(255, 0, 0);
    pub const GREEN: Self = Self::from_rgb(0, 255, 0);
    pub const BLUE: Self = Self::from_rgb(0, 0, 255);
}
