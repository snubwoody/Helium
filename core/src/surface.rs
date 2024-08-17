use glium::{
	glutin::surface::WindowSurface, 
	index, 
	Blend, 
	DrawParameters, 
	Surface as GliumSurface, 
	VertexBuffer
};
use crate::{app::view::RenderContext, colour::Colour, utils::Bounds, vertex::Vertex};

pub trait Surface {
	fn draw(
		&mut self,
		display:&glium::Display<WindowSurface>,
		frame:&mut glium::Frame,
		window:&winit::window::Window,
		program:&RenderContext
	);

	/// Set the position of the [`Widget`]
	fn position(&mut self, x:f32,y:f32);	
	
	/// Get the [`Widget`] position
	fn get_position(&self) -> (f32,f32);

	/// Set the size of the [`Widget`]
	fn size(&mut self,width:u32,height:u32);

	/// Get the size of the [`Widget`]
	fn get_size(&self) -> (u32,u32);

	/// Get the bounds of the [`Widget`]
	fn get_bounds(&self) -> Bounds;
}

/// This is a primitive that draws to the screen. This holds
/// essential information about the [`Widget`], ie.
/// the colour, coordinates and size.
// TODO change x and y to position
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct RectSurface{
	pub x:f32,
	pub y:f32,
	pub width:u32,
	pub height:u32,
	pub colour:Colour,
}

impl RectSurface {
	pub fn new(x:f32,y:f32,width:u32,height:u32,colour:Colour) -> Self{
		Self { x,y,width,height,colour }
	}

	pub fn render(
		&mut self,
		display:&glium::Display<WindowSurface>,
		frame:&mut glium::Frame,
		window:&winit::window::Window,
		program:&glium::Program,
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
			&program, 
			&uniform! {
				width:screen_width,
				height:screen_height,
			},
			&params
		).unwrap();
	}

	pub fn to_vertices(&self) -> Vec<Vertex>{

		let colour = self.colour.normalize();
		let x = self.x as i32;
		let y = self.y as i32;

		let vertex1 = Vertex::new(x, y,colour); //Top left
		let vertex2 = Vertex::new(x+self.width as i32, y,colour); // Top right
		let vertex3 = Vertex::new(x, y+self.height as i32,colour); //Bottom left
		let vertex4 = Vertex::new(x+self.width as i32, y,colour); //Top right
		let vertex5 = Vertex::new(x, y+self.height as i32,colour); // Bottom left
		let vertex6 = Vertex::new(x+self.width as i32, y+self.height as i32,colour); //Bottom right

		return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
	}
}

impl Surface for RectSurface {
	fn draw(
		&mut self,
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

	fn size(&mut self,width:u32,height:u32){
		self.width = width;
		self.height = height;
	} 

	fn get_size(&self) -> (u32,u32){
		(self.width,self.height)
	}

	fn get_bounds(&self) -> Bounds{
		let position = self.get_position();
		let size = self.get_size();
		Bounds{
			x:[position.0,size.0 as f32],
			y:[position.1,size.1 as f32],
		}
	}
}

impl Default for RectSurface {
	fn default() -> Self {
		Self { 
			x:0.0, 
			y:0.0, 
			width:0, 
			height:0, 
			colour:Colour::Rgb(255, 255, 255) 
		}
	}
}
