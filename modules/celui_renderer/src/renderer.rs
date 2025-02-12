use crate::types::Vertex;

use celui_collections::collections::Vec;

// ------------------------------- Renderer -------------------------------- //

pub struct Renderer {
    pub(crate) vertices: Vec<Vertex>,
    pub(crate) indices: Vec<usize>,
}
