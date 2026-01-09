use wgpu::{SurfaceTexture, TextureView};

pub struct Frame {
    pub(crate) texture: SurfaceTexture,
    pub(crate) view: TextureView,
    pub(crate) cleared: bool,
}

impl Frame {
    pub fn new(texture: SurfaceTexture) -> Self {
        let view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        Self {
            texture,
            view,
            cleared: false,
        }
    }
}
