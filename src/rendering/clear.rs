use crate::{gamma::Gamma, rendering::Frame};

impl<S> Gamma<S> {
    pub fn clear_screen(&mut self, r: u8, g: u8, b: u8) {
        let (surface, device, queue) = match (
            self.surface.as_ref(),
            self.device.as_ref(),
            self.queue.as_ref(),
        ) {
            (Some(s), Some(d), Some(q)) => (s, d, q),
            _ => return,
        };

        if self.current_frame.is_none() {
            let texture = match surface.get_current_texture() {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Failed to get surface texture: {:?}", e);
                    return;
                }
            };

            self.current_frame = Some(Frame::new(texture));
        }

        let frame = self.current_frame.as_mut().unwrap();

        if !frame.cleared {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Clear Encoder"),
            });

            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Clear Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: r as f64 / 255.0,
                                g: g as f64 / 255.0,
                                b: b as f64 / 255.0,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                });
            }

            queue.submit(std::iter::once(encoder.finish()));
            frame.cleared = true;
        }
    }
}
