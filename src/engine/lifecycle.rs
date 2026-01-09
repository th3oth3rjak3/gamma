use std::time::Instant;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::WindowId,
};

use crate::{gamma::Gamma, rendering::context::initialize_rendering};

impl<S> Gamma<S> {
    pub fn run(mut self: Self) -> Result<(), String> {
        let event_loop = EventLoop::new().expect("Error occurred starting the event loop");
        event_loop.set_control_flow(ControlFlow::Poll);

        // Set the first frame time just before running the game
        self.last_frame_time = Instant::now();
        event_loop.run_app(&mut self).map_err(|err| err.to_string())
    }
}

impl<S> ApplicationHandler for Gamma<S> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return; // Already initialized
        }

        initialize_rendering(self, event_loop);
        let init_fn = self.init_fn;
        self.state = init_fn(self);

        if let Some(window) = &self.window {
            window.request_redraw();
        }
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
                self.state = None;

                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let update_fn = self.update_fn;
                let draw_fn = self.draw_fn;

                let mut state = self.state.take();

                // Call the user's update function to update the game state.
                update_fn(self, state.as_mut());

                // Call the user's draw function to prepare to draw to the window.
                draw_fn(self, state.as_mut());

                // Put the state back where it belongs
                self.state = state;

                if let Some(frame) = self.current_frame.take() {
                    frame.texture.present();
                }

                // Only request redraw if still running and window exists
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}
