use winit::dpi::LogicalSize;

use crate::gamma::Gamma;

pub type UpdateFn<S> = fn(&mut Gamma<S>, &mut S);
pub type DrawFn<S> = fn(&mut Gamma<S>, &mut S);
pub type InitFn<S> = fn(&mut Gamma<S>) -> S;

pub struct GammaBuilder<S> {
    // User Provided
    pub(crate) draw_fn: Option<DrawFn<S>>,
    pub(crate) update_fn: Option<UpdateFn<S>>,
    pub(crate) init_fn: Option<InitFn<S>>,

    // Configuration Options
    pub(crate) title: Option<String>,
    pub(crate) logical_size: Option<LogicalSize<f64>>,
    pub(crate) resizable: bool,
    pub(crate) vsync: bool,
    pub(crate) fullscreen: bool,
}

impl<S> GammaBuilder<S> {
    pub fn new() -> Self {
        Self {
            // User Provided
            draw_fn: None,
            update_fn: None,
            init_fn: None,

            // Configuration
            title: None,
            logical_size: None,
            resizable: true,
            vsync: true,
            fullscreen: false,
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

    pub fn with_fullscreen(mut self: Self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    pub fn on_init(mut self: Self, init: InitFn<S>) -> Self {
        self.init_fn = Some(init);
        self
    }

    pub fn on_draw(mut self: Self, draw: DrawFn<S>) -> Self {
        self.draw_fn = Some(draw);
        self
    }

    pub fn on_update(mut self: Self, update: UpdateFn<S>) -> Self {
        self.update_fn = Some(update);
        self
    }
}
