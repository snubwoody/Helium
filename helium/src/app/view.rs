use super::{events::EventQueue, AppState};
use crate::widgets::{Text, Widget, WidgetBody};
use crystal::LayoutSolver;
use std::{sync::{Arc, Mutex, RwLock}, thread, time::{Duration, Instant}};
use winit::window::Window;

pub struct View {
    root_layout: Box<dyn crystal::Layout>,
    root_widget: Arc<RwLock<dyn Widget>>,
    root_body: WidgetBody,
    event_queue: EventQueue,
}

impl View {
    pub fn new(root_widget: impl Widget + 'static, event_queue: EventQueue) -> Self {
        let (root_body, root_layout) = root_widget.build();

        Self {
            root_body,
            root_layout,
            root_widget: Arc::new(RwLock::new(root_widget)),
            event_queue,
        }
    }

	pub fn setup_loop(&mut self){
		let widget = self.root_widget.clone();		
		async_std::task::spawn(async move {
			widget.write().unwrap().update();
		});
	}

	pub fn update(&mut self){
		match self.root_widget.try_read() {
			Ok(widget) => {
				let (body,layout) = widget.build();
				self.root_body = body;
				self.root_layout = layout;
			},
			Err(_) => {}
		}
	}

    pub fn handle_events(&mut self, event: winit::event::WindowEvent, window: &Window) {
        // Pass the events to the event manager to determine which events have fired
        // for which widgets.
        self.event_queue.handle_events(&event, &self.root_body);
        window.request_redraw();
    }

    pub fn render(&mut self, state: &AppState) {
        let now = Instant::now();
        let output = state.surface.get_current_texture().unwrap(); // TODO maybe handle this error
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

		
        // Has to be in this order otherwise it crashes particularly because of 0 size textures
        // FIXME above
		self.update();
		let _ = LayoutSolver::solve(&mut *self.root_layout, state.size);
        
		self.root_body.update_sizes(&*self.root_layout);
        self.root_body.render(&mut render_pass, state);

        // Drop the render pass because it borrows encoder mutably
        std::mem::drop(render_pass);

        state.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        log::debug!("{}ms", now.elapsed().as_millis())
    }
}
