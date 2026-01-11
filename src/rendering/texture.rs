// Add to your lib.rs or a new textures.rs module

use wgpu::{Sampler, Texture as WgpuTexture, TextureView, util::DeviceExt};

use crate::{gamma::Gamma, rendering::Frame};

pub struct Texture {
    pub(crate) texture: WgpuTexture,
    pub(crate) view: TextureView,
    pub(crate) sampler: Sampler,
    pub width: u32,
    pub height: u32,
}

impl<S> Gamma<S> {
    // Load from file path
    pub fn load_texture(&self, path: &str) -> Result<Texture, String> {
        let img_bytes = std::fs::read(path)
            .map_err(|e| format!("Failed to read image file '{}': {}", path, e))?;

        self.load_texture_from_bytes(&img_bytes)
    }

    // Load from bytes (works with include_bytes!)
    pub fn load_texture_from_bytes(&self, bytes: &[u8]) -> Result<Texture, String> {
        let device = self.device.as_ref().ok_or("Device not initialized")?;
        let queue = self.queue.as_ref().ok_or("Queue not initialized")?;

        let img = image::load_from_memory(bytes)
            .map_err(|e| format!("Failed to decode image: {}", e))?
            .to_rgba8();

        let dimensions = img.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Loaded Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        Ok(Texture {
            texture,
            view,
            sampler,
            width: dimensions.0,
            height: dimensions.1,
        })
    }

    // Draw a texture at a position
    pub fn draw_texture(&mut self, texture: &Texture, x: f32, y: f32) {
        self.draw_texture_scaled(texture, x, y, texture.width as f32, texture.height as f32);
    }

    pub fn draw_texture_scaled(
        &mut self,
        texture: &Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        let (surface, device, queue, pipeline) = match (
            self.surface.as_ref(),
            self.device.as_ref(),
            self.queue.as_ref(),
            self.texture_pipeline.as_ref(),
        ) {
            (Some(s), Some(d), Some(q), Some(p)) => (s, d, q, p),
            _ => return,
        };

        // Get or create current frame
        if self.current_frame.is_none() {
            let tex = match surface.get_current_texture() {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Failed to get surface texture: {:?}", e);
                    return;
                }
            };
            self.current_frame = Some(Frame::new(tex));
        }

        let frame = self.current_frame.as_ref().unwrap();
        let window_size = self.window.as_ref().unwrap().inner_size();
        let window_width = window_size.width as f32;
        let window_height = window_size.height as f32;

        let ndc_x = (x / window_width) * 2.0 - 1.0;
        let ndc_y = 1.0 - (y / window_height) * 2.0;
        let ndc_width = (width / window_width) * 2.0;
        let ndc_height = (height / window_height) * 2.0;

        #[repr(C)]
        #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }

        let vertices = [
            Vertex {
                position: [ndc_x, ndc_y],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [ndc_x + ndc_width, ndc_y],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [ndc_x, ndc_y - ndc_height],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [ndc_x + ndc_width, ndc_y - ndc_height],
                tex_coords: [1.0, 1.0],
            },
        ];

        let indices: [u16; 6] = [0, 1, 2, 1, 3, 2];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Use the cached pipeline!
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &pipeline.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Texture Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Texture Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&pipeline.pipeline);
            render_pass.set_bind_group(0, &bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..6, 0, 0..1);
        }

        queue.submit(std::iter::once(encoder.finish()));
    }
}
