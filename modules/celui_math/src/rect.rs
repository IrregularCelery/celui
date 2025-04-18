// ------------------------------- rect.rs --------------------------------- //

//! Rectangle representation and geometric/spatial utilities.
//!
//! Defines the `Rect` struct, providing a foundation for calculating various
//! geometric properties and spatial relationships of rectangles, such as
//! intersection, containment, and more.

// --------------------------------- Rect ---------------------------------- //

#[derive(Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    #[inline(always)]
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
