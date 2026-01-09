//https://docs.rs/winit/latest/winit/

use std::sync::Arc;
use std::time::{Duration, Instant};
use wgpu::{
    Adapter, Device, Instance, PresentMode, Queue, Surface, SurfaceConfiguration, TextureFormat,
    TextureUsages,
};

use winit::{
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

pub struct Gamma<S>
where
    S: Default,
{
    pub state: S,

    // Internal
    last_frame_time: std::time::Instant,

    // User Provided
    draw_fn: DrawFn<S>,
    update_fn: UpdateFn<S>,

    // Configuration Options
    title: Option<String>,
    logical_size: Option<LogicalSize<f64>>,
    resizable: bool,
    vsync: bool,

    // Rendering
    window: Option<Arc<Window>>,
    instance: Option<Instance>,
    surface: Option<Surface<'static>>,
    surface_config: Option<SurfaceConfiguration>,
    device: Option<Device>,
    queue: Option<Queue>,
    adapter: Option<Adapter>,
}

impl<S> Default for Gamma<S>
where
    S: Default,
{
    fn default() -> Self {
        Self {
            state: Default::default(),

            // Internal
            last_frame_time: Instant::now(),

            // User Provided
            draw_fn: |_| {},
            update_fn: |_| {},

            // Configuration Options
            title: None,
            logical_size: None,
            resizable: true,
            vsync: true,

            // Rendering
            window: Default::default(),
            instance: None,
            surface: None,
            surface_config: None,
            device: None,
            queue: None,
            adapter: None,
        }
    }
}

impl<S> Gamma<S>
where
    S: Default,
{
    pub fn new(state: S) -> Self {
        Self {
            state,

            // Internal
            last_frame_time: Instant::now(),

            // User Provided
            draw_fn: |_| {},
            update_fn: |_| {},

            // Configuration
            title: None,
            logical_size: None,
            resizable: true,
            vsync: true,

            // Rendering
            window: Default::default(),
            instance: None,
            surface: None,
            surface_config: None,
            device: None,
            queue: None,
            adapter: None,
        }
    }

    pub fn with_title(mut self: Self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn with_size(mut self: Self, width: u16, height: u16) -> Self {
        self.logical_size = Some(LogicalSize::new(width.into(), height.into()));
        self
    }

    pub fn with_resizable(mut self: Self, is_resizable: bool) -> Self {
        self.resizable = is_resizable;
        self
    }

    pub fn with_vsync(mut self: Self, use_vsync: bool) -> Self {
        self.vsync = use_vsync;
        self
    }

    pub fn on_draw(mut self: Self, draw: DrawFn<S>) -> Self {
        self.draw_fn = draw;
        self
    }

    pub fn on_update(mut self: Self, update: UpdateFn<S>) -> Self {
        self.update_fn = update;
        self
    }

    pub fn clear_screen(&mut self, r: u8, g: u8, b: u8) {
        // Use pattern matching to safely unwrap all resources
        let (surface, device, queue) = match (
            self.surface.as_ref(),
            self.device.as_ref(),
            self.queue.as_ref(),
        ) {
            (Some(s), Some(d), Some(q)) => (s, d, q),
            _ => return, // If any resource is missing, bail out
        };

        // Get current texture with error handling
        let frame = match surface.get_current_texture() {
            Ok(f) => f,
            Err(e) => {
                // Surface can fail during shutdown - this is expected
                eprintln!("Failed to get surface texture: {:?}", e);
                return;
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
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
        frame.present();
    }

    pub fn run(mut self: Self) -> Result<(), String> {
        let event_loop = EventLoop::new().expect("Error occurred starting the event loop");
        event_loop.set_control_flow(ControlFlow::Poll);

        // Set the first frame time just before running the game
        self.last_frame_time = Instant::now();
        event_loop.run_app(&mut self).map_err(|err| err.to_string())
    }

    pub fn delta_time(self: &Self) -> Duration {
        Instant::now() - self.last_frame_time
    }
}

impl<S> ApplicationHandler for Gamma<S>
where
    S: Default,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let title = match &self.title {
            Some(title) => title,
            None => "Gamma Game",
        };

        let window_size = match &self.logical_size {
            Some(size) => *size,
            None => LogicalSize::new(800.0, 600.0),
        };

        let window_attributes = Window::default_attributes()
            .with_title(title)
            .with_inner_size(window_size)
            .with_resizable(self.resizable);

        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());

        // Create window
        self.window = Some(window.clone());

        let size = &self.window.as_ref().unwrap().inner_size();

        // Initialize wgpu
        let instance = Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .unwrap();

        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).unwrap();

        let present_mode = if self.vsync {
            PresentMode::AutoVsync
        } else {
            PresentMode::AutoNoVsync
        };

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            view_formats: vec![TextureFormat::Bgra8UnormSrgb],
            desired_maximum_frame_latency: 2,
            width: size.width,
            height: size.height,
            present_mode,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &config);

        self.instance = Some(instance);
        self.surface = Some(surface);
        self.device = Some(device);
        self.queue = Some(queue);
        self.adapter = Some(adapter);
        self.surface_config = Some(config);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                self.surface = None;
                self.device = None;
                self.queue = None;
                self.adapter = None;
                self.instance = None;
                self.surface_config = None;
                self.window = None;
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let update_fn = self.update_fn;
                let draw_fn = self.draw_fn;

                // Call the user's update function to update the game state.
                update_fn(self);

                // Call the user's draw function to prepare to draw to the window.
                draw_fn(self);

                // Update timing and request next frame
                self.last_frame_time = Instant::now();

                // Only request redraw if still running and window exists
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}

// Type aliases to make the function signatures cleaner.
pub type UpdateFn<S> = fn(&mut Gamma<S>);
pub type DrawFn<S> = fn(&mut Gamma<S>);
