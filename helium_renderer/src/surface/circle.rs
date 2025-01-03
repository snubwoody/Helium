use wgpu::util::DeviceExt;
use crate::{
	geometry::RenderContext,
	app::AppState, impl_surface, surface::Surface, 
	geometry::vertex::Vertex, Bounds, Color, Position, Size
};

/// This is a primitive that draws to the screen. This holds
/// essential information about the [`Widget`], ie.
/// the color, coordinates and size.
#[derive(Debug,Clone,PartialEq,Default)]
pub struct CircleSurface{
	pub position:Position,
	pub size:Size,
	pub color:Color,
}

impl CircleSurface {
	pub fn new(radius:u32,color:Color) -> Self{
		let size = Size::new(radius as f32, radius as f32);
		let position = Position::default();
		Self { position,size,color }
	}

	pub fn color(&mut self,color:Color) {
		self.color = color
	}

	pub fn to_vertices(&self) -> Vec<Vertex>{
		let color = self.color.normalize();
		let x = self.position.x;
		let y = self.position.y;

		let vertex1 = Vertex::new(x, y,color); //Top left
		let vertex2 = Vertex::new(x+self.size.width, y,color); // Top right
		let vertex3 = Vertex::new(x, y+self.size.height,color); //Bottom left
		let vertex4 = Vertex::new(x+self.size.width, y,color); //Top right
		let vertex5 = Vertex::new(x, y+self.size.height,color); // Bottom left
		let vertex6 = Vertex::new(x+self.size.width, y+self.size.height,color); //Bottom right

		return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
	}

}

impl Surface for CircleSurface {
	fn draw(
		&mut self,
		render_pass:&mut wgpu::RenderPass,
		context: &RenderContext,
		state: &AppState
	) {
		let vertices = self.to_vertices();
		
		let vertex_buffer = state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
			label: Some("Vertex buffer"),
			contents: bytemuck::cast_slice(&vertices),
			usage: wgpu::BufferUsages::VERTEX,
		});

		state.queue.write_buffer(
            &state.context.circle_pipeline.position_buffer,
            0,
            bytemuck::cast_slice(&[self.position.x,self.position.y]),
        );
		state.queue.write_buffer(
            &state.context.circle_pipeline.diameter_buffer,
            0,
            bytemuck::cast_slice(&[self.size.width]),
        );

		// Set the render pipeline and vertex buffer
		render_pass.set_pipeline(&context.circle_pipeline.pipeline);
		render_pass.set_bind_group(0, &context.circle_pipeline.window_uniform.bind_group(), &[]);
		render_pass.set_bind_group(1, &context.circle_pipeline.bounds_bind_group, &[]);
		render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
		render_pass.draw(0..vertices.len() as u32, 0..1);
	}

	impl_surface!();
}
