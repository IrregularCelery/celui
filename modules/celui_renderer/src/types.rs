use celui_math::{Color, Vec2};

// -------------------------------- Vertex --------------------------------- //

pub(crate) struct Vertex {
    pub position: Vec2,
    pub color: Color,
    pub uv: Vec2,
    pub texture_id: TextureId,
}

// ------------------------------ TextureId -------------------------------- //

pub(crate) struct TextureId(pub usize);

pub(crate) struct Texture {
    id: TextureId,
}

// TODO: Move this to the backend crate
pub trait GraphicsBackend {}
