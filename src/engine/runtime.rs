use winit::{
    application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop,
    window::WindowId,
};

use crate::{gamma::Gamma, rendering::context::initialize_rendering};

pub struct GammaRuntime<S> {
    pub(crate) context: Gamma<S>,
    pub(crate) state: Option<S>,
}

impl<S> ApplicationHandler for GammaRuntime<S> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.context.window.is_some() {
            return; // Already initialized
        }

        initialize_rendering(&mut self.context, event_loop);

        let init = self.context.init_fn.unwrap();
        self.state = Some(init(&mut self.context));

        if let Some(window) = &self.context.window {
            window.request_redraw();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");

                self.context.surface = None;
                self.context.device = None;
                self.context.queue = None;
                self.context.adapter = None;
                self.context.instance = None;
                self.context.surface_config = None;
                self.context.window = None;
                self.context.texture_pipeline = None;
                self.state = None;

                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let update_fn = self.context.update_fn;
                let draw_fn = self.context.draw_fn;

                let mut state = self.state.as_mut().expect("state not found when expected");

                // Call the user's update function to update the game state.
                update_fn(&mut self.context, &mut state);

                // Call the user's draw function to prepare to draw to the window.
                draw_fn(&mut self.context, &mut state);

                if let Some(frame) = self.context.current_frame.take() {
                    frame.texture.present();
                }

                // Only request redraw if still running and window exists
                if let Some(window) = &self.context.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}
