use std::sync::Arc;
use std::time::Instant;
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};

use winit::{dpi::LogicalSize, window::Window};

use crate::{
    builder::InitFn,
    rendering::{TexturePipeline, frame::Frame},
};

pub type UpdateFn<S> = fn(&mut Gamma<S>, &mut S);
pub type DrawFn<S> = fn(&mut Gamma<S>, &mut S);

pub struct Gamma<S> {
    // Internal
    pub(crate) last_frame_time: std::time::Instant,
    pub(crate) current_frame: Option<Frame>,

    // User Provided
    pub(crate) draw_fn: DrawFn<S>,
    pub(crate) update_fn: UpdateFn<S>,
    pub(crate) init_fn: Option<InitFn<S>>,

    // Configuration
    pub(crate) title: String,
    pub(crate) logical_size: LogicalSize<f64>,
    pub(crate) resizable: bool,
    pub(crate) vsync: bool,
    pub(crate) fullscreen: bool,

    // Rendering
    pub(crate) window: Option<Arc<Window>>,
    pub(crate) instance: Option<Instance>,
    pub(crate) surface: Option<Surface<'static>>,
    pub(crate) surface_config: Option<SurfaceConfiguration>,
    pub(crate) device: Option<Device>,
    pub(crate) queue: Option<Queue>,
    pub(crate) adapter: Option<Adapter>,
    pub(crate) texture_pipeline: Option<TexturePipeline>,
}

impl<S> Default for Gamma<S> {
    fn default() -> Self {
        Self {
            // Internal
            last_frame_time: Instant::now(),
            current_frame: None,

            // User Provided
            draw_fn: |_, _| {},
            update_fn: |_, _| {},
            init_fn: None,

            // Configuration
            title: String::new(),
            logical_size: LogicalSize {
                width: 800.0,
                height: 600.0,
            },
            resizable: true,
            vsync: true,
            fullscreen: false,

            // Rendering
            window: Default::default(),
            instance: None,
            surface: None,
            surface_config: None,
            device: None,
            queue: None,
            adapter: None,
            texture_pipeline: None,
        }
    }
}
