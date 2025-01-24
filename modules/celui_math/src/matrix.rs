// --------------------------------- Mat2 ---------------------------------- //

pub struct Mat2 {
    pub elements: [f32; 4],
}

impl Mat2 {
    #[inline(always)]
    pub const fn identity() -> Self {
        Self {
            elements: [1.0, 0.0, 0.0, 1.0],
        }
    }
}

// --------------------------------- Mat3 ---------------------------------- //

pub struct Mat3 {
    pub elements: [f32; 9],
}

impl Mat3 {
    #[inline(always)]
    pub const fn identity() -> Self {
        Self {
            elements: [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        }
    }
}

// --------------------------------- Mat4 ---------------------------------- //

pub struct Mat4 {
    pub elements: [f32; 16],
}

impl Mat4 {
    #[inline(always)]
    pub const fn identity() -> Self {
        Self {
            elements: [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        }
    }
}
