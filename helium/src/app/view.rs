use std::time::Instant;
use crystal::LayoutSolver;
use winit::window::Window;
use crate::widgets::{Widget, WidgetBody};
use super::{events::EventQueue, AppState};

pub struct View{
	root_layout:Box<dyn crystal::Layout>,
	_root_widget:Box<dyn Widget>,
	root_body:WidgetBody,
	event_queue:EventQueue,
}

impl View {
	pub fn new(root_widget:impl Widget + 'static,event_queue:EventQueue) -> Self {
		let (root_body,root_layout) = root_widget.build();
		
		Self { 
			root_body,
			root_layout,
			_root_widget:Box::new(root_widget),
			event_queue,
		}
	}

	pub fn handle_events(&mut self,event: winit::event::WindowEvent,window:&Window){
		// Pass the events to the event manager to determine which events have fired
		// for which widgets.
		self.event_queue.handle_events(&event,&self.root_body);
		window.request_redraw();
	}

	pub fn render(&mut self,state:&AppState) {
		let now = Instant::now();
		let output = state.surface.get_current_texture().unwrap(); // TODO maybe handle this error
		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
		
		let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
			label:Some("Render encoder")
		});
		
		let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
			label: Some("Render Pass"),
			color_attachments: &[Some(wgpu::RenderPassColorAttachment {
				view: &view,
				resolve_target: None,
				ops: wgpu::Operations {
					load: wgpu::LoadOp::Clear(wgpu::Color{r: 1.0,g: 1.0,b: 1.0,a:1.0}),
					store: wgpu::StoreOp::Store,
				},
			})],
			depth_stencil_attachment: None,
			occlusion_query_set: None,
			timestamp_writes: None,
		});
		
		
		let _ = LayoutSolver::solve(&mut *self.root_layout, state.size);
		
		// Has to be in this order otherwise it crashes particularly because of 0 size textures
		// FIXME above
		self.root_body.update_sizes(&self.root_layout);
		self.root_body.render(&mut render_pass,state);
		
		// for error in errors{
		// 	log::warn!("{error}")
		// }

		// Drop the render pass because it borrows encoder mutably
		std::mem::drop(render_pass);
	
		state.queue.submit(std::iter::once(encoder.finish()));
		output.present();
		log::debug!("{}fps",1000/now.elapsed().as_millis())
	}
}




