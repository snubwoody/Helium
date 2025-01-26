pub mod vertex;
pub mod builders;
mod pipeline;
mod error;
pub mod primitives;
pub use error::Error;
use std::rc::Rc;
use helium_core::{
	color::*, 
	Size
};
use pipeline::{GlobalResources, RectPipeline};
use primitives::{IntoPrimitive, Primitive, Rect};
use vertex::Vertex;
use winit::window::Window;


pub struct Renderer<'r> {
    surface: wgpu::Surface<'r>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
	global:Rc<GlobalResources>,
	rect_pipeline:RectPipeline,
	draw_queue:Vec<Primitive>
}

impl<'r> Renderer<'r> {
    pub async fn new(window: &'r Window) -> Self {
        let size = Size::from(window.inner_size());

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap(); // FIXME return these errors

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

		let global = Rc::new(GlobalResources::new(&device, Size::from(window.inner_size())));
		let rect_pipeline = RectPipeline::new(&device,config.format,Rc::clone(&global));
        
		Self {
            surface,
            device,
            queue,
            config,
			rect_pipeline,
			global,
			draw_queue:vec![]
        }
    }

    pub fn resize(&mut self, size: Size) {
        self.config.width = size.width as u32;
        self.config.height = size.height as u32;
        
		// Resize the surface with the window to keep the right scale
		self.queue.write_buffer(
			self.global.window_buffer(), 0, bytemuck::cast_slice(&[size])
		);
		self.surface.configure(&self.device, &self.config);
    }

	/// Add primitives to the draw queue
	pub fn draw<I,P>(&mut self,primitives:I)
	where 
		I:IntoIterator<Item = P>,
		P:IntoPrimitive
	{
		self.draw_queue.extend(
			primitives.into_iter().map(|p|p.into_primitive())
		);
	}

	/// TODO Change to present then add primitive queue?
	pub fn render(&mut self){
		let instant = std::time::Instant::now();
		
		let output = self.surface.get_current_texture().unwrap(); // TODO maybe handle this error
		let view = output
			.texture
			.create_view(&wgpu::TextureViewDescriptor::default());

		let mut encoder = self
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

		for primitive in self.draw_queue.drain(..){
			match primitive {
				Primitive::Rect(rect) => {
					self.rect_pipeline.draw(&rect, &self.device, &mut render_pass);
				}
			}
		}
		
		// Drop the render pass because it borrows encoder mutably
		std::mem::drop(render_pass);

		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();
		
		//log::trace!("Frame time: {:?}",instant.elapsed());
	}

}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn create_renderer(){
		todo!()
	}
}
