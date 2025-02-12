// -------------------------- PrimitiveRenderer ---------------------------- //

use crate::renderer::Renderer;

pub trait PrimitiveRenderer {
    fn draw_triangle(&mut self);
    fn draw_rectangle(&mut self);
}

impl PrimitiveRenderer for Renderer {
    #[inline(always)]
    fn draw_triangle(&mut self) {}

    #[inline(always)]
    fn draw_rectangle(&mut self) {}
}
