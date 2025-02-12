use celui_math::{Color, Vec2};

// -------------------------------- Vertex --------------------------------- //

pub struct Vertex {
    pub position: Vec2,
    pub color: Color,
    pub uv: Vec2,
    pub texture_id: TextureId,
}

// ------------------------------ TextureId -------------------------------- //

pub struct TextureId(usize);
