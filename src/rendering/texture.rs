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

    // Draw a texture with custom width/height
    pub fn draw_texture_scaled(
        &mut self,
        texture: &Texture,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) {
        let (surface, device, queue) = match (
            self.surface.as_ref(),
            self.device.as_ref(),
            self.queue.as_ref(),
        ) {
            (Some(s), Some(d), Some(q)) => (s, d, q),
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

        // Get window dimensions for coordinate conversion
        let window_size = self.window.as_ref().unwrap().inner_size();
        let window_width = window_size.width as f32;
        let window_height = window_size.height as f32;

        // Convert screen coordinates to normalized device coordinates (-1 to 1)
        // Note: in NDC, Y goes from -1 (bottom) to 1 (top)
        let ndc_x = (x / window_width) * 2.0 - 1.0;
        let ndc_y = 1.0 - (y / window_height) * 2.0; // Flip Y
        let ndc_width = (width / window_width) * 2.0;
        let ndc_height = (height / window_height) * 2.0;

        // Create vertices for a quad
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
            }, // Top-left
            Vertex {
                position: [ndc_x + ndc_width, ndc_y],
                tex_coords: [1.0, 0.0],
            }, // Top-right
            Vertex {
                position: [ndc_x, ndc_y - ndc_height],
                tex_coords: [0.0, 1.0],
            }, // Bottom-left
            Vertex {
                position: [ndc_x + ndc_width, ndc_y - ndc_height],
                tex_coords: [1.0, 1.0],
            }, // Bottom-right
        ];

        let indices: [u16; 6] = [0, 1, 2, 1, 3, 2];

        // Create buffers
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

        // Create shader (we'll define this below)
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Texture Shader"),
            source: wgpu::ShaderSource::Wgsl(TEXTURE_SHADER.into()),
        });

        // Create bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &bind_group_layout,
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

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Texture Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            immediate_size: 0,
        });

        // Create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Texture Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: self.surface_config.as_ref().unwrap().format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
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
                        load: wgpu::LoadOp::Load, // Don't clear, preserve what's already there
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&render_pipeline);
            render_pass.set_bind_group(0, &bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..6, 0, 0..1);
        }

        queue.submit(std::iter::once(encoder.finish()));
    }
}

// Shader code
const TEXTURE_SHADER: &str = r#"
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
}

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(input.position, 0.0, 1.0);
    out.tex_coords = input.tex_coords;
    return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
"#;
