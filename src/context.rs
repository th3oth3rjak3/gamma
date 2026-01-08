//https://docs.rs/winit/latest/winit/

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

pub struct GammaContext<S>
where
    S: Default,
{
    pub state: S,
    window: Option<Window>,
    draw_fn: DrawFn<S>,
    update_fn: UpdateFn<S>,
}

impl<S> Default for GammaContext<S>
where
    S: Default,
{
    fn default() -> Self {
        Self {
            state: Default::default(),
            window: Default::default(),
            draw_fn: |_| {},
            update_fn: |_| {},
        }
    }
}

impl<S> GammaContext<S>
where
    S: Default,
{
    pub fn new(state: S) -> Self {
        Self {
            state,
            window: Default::default(),
            draw_fn: |_| {},
            update_fn: |_| {},
        }
    }

    pub fn draw(mut self: Self, draw: DrawFn<S>) -> Self {
        self.draw_fn = draw;
        self
    }

    pub fn update(mut self: Self, update: UpdateFn<S>) -> Self {
        self.update_fn = update;
        self
    }

    pub fn run(mut self: Self) -> Result<(), String> {
        let event_loop = EventLoop::new().expect("Error occurred starting the event loop");
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(&mut self).map_err(|err| err.to_string())
    }
}

impl<S> ApplicationHandler for GammaContext<S>
where
    S: Default,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let update_fn = self.update_fn;
                let draw_fn = self.draw_fn;
                update_fn(self);
                draw_fn(self);
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

// Type aliases to make the function signatures cleaner.
type UpdateFn<S> = fn(&mut GammaContext<S>);
type DrawFn<S> = fn(&mut GammaContext<S>);
