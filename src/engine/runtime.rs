use std::time::Instant;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowId,
};

use crate::{gamma::Gamma, rendering::context::initialize_rendering};

pub struct GammaRuntime<S> {
    pub(crate) context: Gamma<S>,
    pub(crate) state: Option<S>,
}

impl<S> GammaRuntime<S> {
    pub(crate) fn shutdown(&mut self, event_loop: &ActiveEventLoop) {
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
                self.shutdown(event_loop);
            }
            WindowEvent::RedrawRequested => {
                self.context.delta = Instant::now() - self.context.last_frame_time;
                self.context.last_frame_time = Instant::now();

                let update_fn = self.context.update_fn;
                let draw_fn = self.context.draw_fn;

                let state = self.state.as_mut().expect("state not found when expected");

                // Call the user's update function to update the game state.
                update_fn(&mut self.context, state);

                // Call the user's draw function to prepare to draw to the window.
                draw_fn(&mut self.context, state);

                // Clear the just_pressed_keys and just_released_keys for the next frame
                self.context.just_pressed_keys.clear();
                self.context.just_released_keys.clear();

                if let Some(frame) = self.context.current_frame.take() {
                    frame.texture.present();
                }

                // Only request redraw if still running and window exists
                if let Some(window) = &self.context.window {
                    window.request_redraw();
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(keycode) = event.physical_key {
                    if event.state.is_pressed()
                        && keycode == KeyCode::Escape
                        && self.context.close_on_escape
                    {
                        self.shutdown(event_loop);
                    }

                    if event.state.is_pressed() {
                        if self.context.pressed_keys.insert(keycode) {
                            self.context.just_pressed_keys.insert(keycode);
                        }
                    } else {
                        if self.context.pressed_keys.remove(&keycode) {
                            self.context.just_released_keys.insert(keycode);
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
