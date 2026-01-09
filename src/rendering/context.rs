use crate::gamma::Gamma;
use std::sync::Arc;
use wgpu::{Instance, PresentMode, SurfaceConfiguration, TextureFormat, TextureUsages};
use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

pub(crate) fn initialize_rendering<S>(gamma: &mut Gamma<S>, event_loop: &ActiveEventLoop) {
    let title = gamma.title.as_deref().unwrap_or("Gamma Game");
    let window_size = gamma
        .logical_size
        .unwrap_or_else(|| LogicalSize::new(800.0, 600.0));

    let window_attributes = Window::default_attributes()
        .with_title(title)
        .with_inner_size(window_size)
        .with_resizable(gamma.resizable);

    let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
    let size = window.inner_size();

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

    let present_mode = if gamma.vsync {
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

    gamma.window = Some(window);
    gamma.instance = Some(instance);
    gamma.surface = Some(surface);
    gamma.device = Some(device);
    gamma.queue = Some(queue);
    gamma.adapter = Some(adapter);
    gamma.surface_config = Some(config);
}
