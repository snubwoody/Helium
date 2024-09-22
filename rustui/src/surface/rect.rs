use glium::{
	glutin::surface::WindowSurface, 
	index, 
	Blend, 
	DrawParameters, 
	Surface as GliumSurface, 
	VertexBuffer
};
use crate::{
	app::view::RenderContext, colour::Colour, surface::Surface, utils::{Bounds, Size}, vertex::Vertex
};

// TODO change x and y to position
// TODO change width and height to size
/// This is a primitive that draws to the screen. This holds
/// essential information about the [`Widget`], ie.
/// the colour, coordinates and size.
#[derive(Debug,Clone,PartialEq)]
pub struct RectSurface{
	pub x:f32,
	pub y:f32,
	pub size:Size,
	pub colour:Colour,
}

impl RectSurface {
	pub fn new(x:f32,y:f32,width:u32,height:u32,colour:Colour) -> Self{
		let size = Size::new(width as f32, height as f32);
		Self { x,y,size,colour }
	}

	pub fn colour(&mut self,colour:Colour) {
		self.colour = colour
	}

	pub fn to_vertices(&self) -> Vec<Vertex>{

		let colour = self.colour.normalize();
		let x = self.x as i32;
		let y = self.y as i32;

		let vertex1 = Vertex::new(x, y,colour); //Top left
		let vertex2 = Vertex::new(x+self.size.width as i32, y,colour); // Top right
		let vertex3 = Vertex::new(x, y+self.size.height as i32,colour); //Bottom left
		let vertex4 = Vertex::new(x+self.size.width as i32, y,colour); //Top right
		let vertex5 = Vertex::new(x, y+self.size.height as i32,colour); // Bottom left
		let vertex6 = Vertex::new(x+self.size.width as i32, y+self.size.height as i32,colour); //Bottom right

		return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
	}

}

impl Surface for RectSurface {
	fn draw(
		&self,
		display:&glium::Display<WindowSurface>,
		frame:&mut glium::Frame,
		window:&winit::window::Window,
		context:&RenderContext
	) {
		let vertices:Vec<Vertex> = self.to_vertices();
		let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
		let indices = index::NoIndices(glium::index::PrimitiveType::TrianglesList);

		let params = DrawParameters{
			blend:Blend::alpha_blending(),
			..Default::default()
		};

		let screen_width = window.inner_size().width as f32;
		let screen_height = window.inner_size().height as f32;

		frame.draw(
			&vertex_buffer, 
			&indices, 
			&context.surface_program, 
			&uniform! {
				width:screen_width,
				height:screen_height,
			},
			&params
		).unwrap();
	}

	fn position(&mut self, x:f32,y:f32){
		self.x = x;
		self.y = y;
	} 
	
	fn get_position(&self) -> (f32,f32){
		(self.x,self.y)
	} 

	fn size(&mut self,width:f32,height:f32){
		self.size.width = width;
		self.size.height = height;
	} 

	fn get_size(&self) -> Size {
		self.size
	}

	fn get_bounds(&self) -> Bounds{
		let position = self.get_position();
		let size = self.get_size();
		Bounds{
			x:[position.0,size.width],
			y:[position.1,size.height],
		}
	}
}

impl Default for RectSurface {
	fn default() -> Self {
		Self { 
			x:0.0, 
			y:0.0, 
			size:Size::new(0.0, 0.0),
			colour:Colour::Rgb(255, 255, 255) 
		}
	}
}
