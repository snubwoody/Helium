pub mod events;
pub mod view;

use crate::{geometry::RenderContext,Size};
use async_std::task;
use view::View;
use winit::{
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

/// This is a singular isolated program. Most projects
/// will only contain one app.
pub struct App {
    event_loop: EventLoop<()>,
    window: Window,
    views: Vec<View>,
    index: usize,
}

impl App {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();

        // Set the event loop to always start a new 
		// iteration even if there are no events.
        event_loop.set_control_flow(ControlFlow::Poll);

        let window = WindowBuilder::new().build(&event_loop).unwrap();

        Self {
            event_loop,
            window,
            views: vec![],
            index: 0,
        }
    }

    pub fn add_view(mut self, view: View) -> Self {
        self.views.push(view);
        self
    }

    pub fn run(mut self) {
        let mut state = task::block_on(AppState::new(&self.window));
		// TODO when the window is minimized the size of the widgets are changing to zero which
		// causing wgpu to panic.
        self.event_loop
        .run(|event, window_target| match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::RedrawRequested => self.views[self.index].render(&state),
                WindowEvent::Resized(size) => {
					state.resize(size);
					self.window.request_redraw();
				},
                event => {self.views[self.index].handle_events(event,&self.window);}
            },
            _ => {}
        })
        .expect("Event loop error occured");
    }
}

pub struct AppState<'a> {
    pub surface: wgpu::Surface<'a>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub context: RenderContext,
	pub config: wgpu::SurfaceConfiguration,
    pub size: Size,
}

impl<'a> AppState<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let size = window.inner_size().into();

        // Handle to wpgu for creating a surface and an adapter
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        // Create the surface to draw on
        let surface = instance.create_surface(window).unwrap();

        // Handle to the graphics card
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        // The device is an open connection to the graphics
        // card and the queue is a command buffer
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device/Queue"),
                    required_features: wgpu::Features::empty(),
                    ..Default::default()
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .find(|s| s.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let context = RenderContext::new(&device, &config, &size);
		
        Self {
            surface,
            device,
            queue,
            context,
			config,
            size,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size.into();
		self.config.width = size.width as u32;
		self.config.height = size.height as u32;
		// Resize the surface with the window to keep the right scale
		self.surface.configure(&self.device, &self.config);
		
		//TODO maybe add a global uniform instead
        self.queue.write_buffer(
            &self.context.rect_pipeline.window_buffer,
            0,
            bytemuck::cast_slice(&[self.size.width, self.size.height]),
        );
        self.queue.write_buffer(
            &self.context.text_pipeline.window_buffer,
            0,
            bytemuck::cast_slice(&[self.size.width, self.size.height]),
        );
        self.queue.write_buffer(
            &self.context.circle_pipeline.window_uniform.buffer(),
            0,
            bytemuck::cast_slice(&[self.size.width, self.size.height]),
        );
        self.queue.write_buffer(
            &self.context.image_pipeline.window_buffer,
            0,
            bytemuck::cast_slice(&[self.size.width, self.size.height]),
        );
        self.queue.write_buffer(
            &self.context.icon_pipeline.window_buffer,
            0,
            bytemuck::cast_slice(&[self.size.width, self.size.height]),
        );
    }
}


