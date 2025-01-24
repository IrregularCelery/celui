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

#[cfg(feature = "std")]
impl std::fmt::Debug for Mat2 {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mat2({:?})", self.elements)
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

#[cfg(feature = "std")]
impl std::fmt::Debug for Mat3 {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mat3({:?})", self.elements)
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

#[cfg(feature = "std")]
impl std::fmt::Debug for Mat4 {
    #[inline(always)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mat4({:?})", self.elements)
    }
}
