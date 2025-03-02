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

pub(crate) struct TextureRegistry {
    /// Default texture used for colored primitives.
    default: Texture,
    /// Texture slots, stores the backend id for the textures.
    slots: Vec<u32>,
    /// Next available texture slot.
    index: usize,
    /// Lookup map of the textures.
    ///
    /// **Key:** Backend texture id.
    /// **Value:** Texture slot index.
    map: std::collections::HashMap<u32, usize>, // TODO: USE `celui_collections::HashMap`
}

impl TextureRegistry {
    #[inline(always)]
    fn reset(&mut self) {
        self.index = 1; // `0` is reserved for the default texture
        self.map.clear();
    }
}

// TODO: Move this to the backend crate
pub trait GraphicsBackend {}
