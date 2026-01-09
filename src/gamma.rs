use std::sync::Arc;
use std::time::Instant;
use wgpu::{Adapter, Device, Instance, Queue, Surface, SurfaceConfiguration};

use winit::{dpi::LogicalSize, window::Window};

use crate::rendering::frame::Frame;

pub type UpdateFn<S> = fn(&mut Gamma<S>, Option<&mut S>);
pub type DrawFn<S> = fn(&mut Gamma<S>, Option<&mut S>);
pub type InitFn<S> = fn(&mut Gamma<S>) -> Option<S>;

pub struct Gamma<S> {
    pub(crate) state: Option<S>,

    // Internal
    pub(crate) last_frame_time: std::time::Instant,
    pub(crate) current_frame: Option<Frame>,

    // User Provided
    pub(crate) draw_fn: DrawFn<S>,
    pub(crate) update_fn: UpdateFn<S>,
    pub(crate) init_fn: InitFn<S>,

    // Configuration Options
    pub(crate) title: Option<String>,
    pub(crate) logical_size: Option<LogicalSize<f64>>,
    pub(crate) resizable: bool,
    pub(crate) vsync: bool,

    // Rendering
    pub(crate) window: Option<Arc<Window>>,
    pub(crate) instance: Option<Instance>,
    pub(crate) surface: Option<Surface<'static>>,
    pub(crate) surface_config: Option<SurfaceConfiguration>,
    pub(crate) device: Option<Device>,
    pub(crate) queue: Option<Queue>,
    pub(crate) adapter: Option<Adapter>,
}

impl<S> Default for Gamma<S> {
    fn default() -> Self {
        Self {
            state: Default::default(),

            // Internal
            last_frame_time: Instant::now(),
            current_frame: None,

            // User Provided
            draw_fn: |_, _| {},
            update_fn: |_, _| {},
            init_fn: |_| None,

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

impl<S> Gamma<S> {
    pub fn new() -> Self {
        Self {
            state: None,

            // Internal
            last_frame_time: Instant::now(),
            current_frame: None,

            // User Provided
            draw_fn: |_, _| {},
            update_fn: |_, _| {},
            init_fn: |_| None,

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

    pub fn on_init(mut self: Self, init: InitFn<S>) -> Self {
        self.init_fn = init;
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
}
