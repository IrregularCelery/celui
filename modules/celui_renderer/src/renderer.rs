use celui_backend::GraphicsBackend;
use celui_collections::collections::Vec;

use crate::types::{TextureRegistry, Vertex};

// ------------------------------- Renderer -------------------------------- //

pub struct Renderer<B: GraphicsBackend> {
    backend: B,

    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<usize>,

    vertex_cursor: usize,
    index_cursor: usize,

    texture_registry: TextureRegistry,
}

impl<B: GraphicsBackend> Renderer<B> {
    pub fn new(backend: B) -> Self {
        todo!()
    }

    fn init(&mut self) {}

    fn reset(&mut self) {}

    fn flush(&mut self) {}

    fn begin(&mut self) {
        self.reset();
    }

    fn end(&mut self) {
        self.flush();
    }
}

impl<B: GraphicsBackend> Drop for Renderer<B> {
    fn drop(&mut self) {}
}
