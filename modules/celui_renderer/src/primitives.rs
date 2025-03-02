use celui_backend::GraphicsBackend;

use crate::renderer::Renderer;

// -------------------------- PrimitiveRenderer ---------------------------- //

pub trait PrimitiveRenderer {
    fn draw_triangle(&mut self);
    fn draw_rectangle(&mut self);
}

impl<B: GraphicsBackend> PrimitiveRenderer for Renderer<B> {
    #[inline(always)]
    fn draw_triangle(&mut self) {}

    #[inline(always)]
    fn draw_rectangle(&mut self) {}
}
