use std::time::Instant;

use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
};

use crate::{builder::GammaBuilder, gamma::Gamma, runtime::GammaRuntime};

impl<S> GammaBuilder<S> {
    pub fn run(self: Self) -> Result<(), String> {
        if self.init_fn.is_none() {
            eprintln!(
                "Cannot call draw or update without init, please register game state with the `on_init` builder method"
            );
            std::process::exit(-1);
        }

        let mut gamma_instance = Gamma::<S> {
            last_frame_time: Instant::now(),
            current_frame: None,
            draw_fn: self.draw_fn.unwrap_or(|_, _| {}),
            update_fn: self.update_fn.unwrap_or(|_, _| {}),
            init_fn: self.init_fn,
            title: self.title.unwrap_or("Gamma Game".into()),
            logical_size: self.logical_size.unwrap_or(LogicalSize {
                width: 800.0,
                height: 600.0,
            }),
            resizable: self.resizable,
            vsync: self.vsync,
            fullscreen: self.fullscreen,
            window: Default::default(),
            instance: None,
            surface: None,
            surface_config: None,
            device: None,
            queue: None,
            adapter: None,
        };

        let event_loop = EventLoop::new().expect("Error occurred starting the event loop");
        event_loop.set_control_flow(ControlFlow::Poll);

        gamma_instance.last_frame_time = Instant::now();

        let mut runtime = GammaRuntime::<S> {
            context: gamma_instance,
            state: None,
        };

        event_loop
            .run_app(&mut runtime)
            .map_err(|err| err.to_string())
    }
}
