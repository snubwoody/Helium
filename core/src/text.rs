use std::io::Cursor;
use text_to_png::{Size, TextRenderer};
use glium::{
	glutin::surface::WindowSurface, 
	index, 
	texture::RawImage2d, 
	Blend, 
	Display, 
	DrawParameters, 
	Surface as GliumSurface, 
	Texture2d, 
	VertexBuffer 
};
use crate::{
	app::view::RenderContext, colour::rgb, surface::Surface, utils::Bounds, vertex::Vertex
};



//FIXME change the colour to Colour enum
/// A rasterized texture of text  
#[derive(Debug)]
pub struct TextSurface{
	x:i32,
	y:i32,
	width:u32,
	height:u32,
	text:String,
	font_size:u8,
	colour:String,
	size:Option<Size>,
	texture:Option<Texture2d>
}

impl TextSurface {
	pub fn new(x:i32,y:i32,text:&str,colour:&str,font_size:u8) -> Self{

		/* let text_renderer = TextRenderer::default();
		let raw_image:RawImage2d<_>;
		let image_size:Size;

		// Render the text as a png
		let text_image = text_renderer.render_text_to_png_data(
			text, 
			font_size, 
			colour
		).unwrap();
		image_size = text_image.size;

		// Get the raw pixel values for the image
		let img = image::load(Cursor::new(text_image.data), image::ImageFormat::Png).unwrap().to_rgba8().into_raw();
		
		// Create an opengl raw image 
		raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&img,(image_size.width,image_size.height));

		// Create the texture from the image
		let texture = glium::texture::Texture2d::new(display, raw_image).unwrap(); */

		
		Self { 
			x, 
			y, 
			width:0,
			height:0,
			text:String::from(text), 
			font_size, 
			colour:String::from(colour),
			size:None, 
			texture:None 
		}
	}
	
	/// Rasterize the text and store the texture 
	pub fn build(&mut self,display:&Display<WindowSurface>) -> &Self{
		let (texture,size) = self.rasterize(display);
		self.texture = Some(texture);
		self.size = Some(size);
		self
	}

	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut glium::Frame,
		window:&winit::window::Window,
		program:&glium::Program,
	) {
		let params = DrawParameters{
			blend:Blend::alpha_blending(),
			..Default::default()
		};

		let screen_width = window.inner_size().width as f32;
		let screen_height = window.inner_size().height as f32;

		
		let texture = self.texture.as_ref().expect("Null texture, call build before render");
		let size = self.size.as_ref().expect("Null size, call build before render");

		
		let uniforms = uniform! {
			width:screen_width,
			height:screen_height,
			tex: texture,
		};

		let vertices:Vec<Vertex> = self.to_vertices(size.width as i32, size.height as i32);
		let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
		let indices = index::NoIndices(glium::index::PrimitiveType::TrianglesList);
		
		frame.draw(
			&vertex_buffer, 
			&indices, 
			&program, 
			&uniforms,
			&params
		).unwrap();
	}

	fn to_vertices(&self,width:i32,height:i32) -> Vec<Vertex>{
		let colour = rgb(255, 255, 255);

		let vertex1 = Vertex::new_with_texture(self.x,self.y,colour,[0.0,1.0]); //Top left
		let vertex2 = Vertex::new_with_texture(self.x+width,self.y,colour,[1.0,1.0]); // Top right
		let vertex3 = Vertex::new_with_texture(self.x, self.y+height,colour,[0.0,0.0]); //Bottom left
		let vertex4 = Vertex::new_with_texture(self.x+width,self.y,colour,[1.0,1.0]); //Top right
		let vertex5 = Vertex::new_with_texture(self.x, self.y+height,colour,[0.0,0.0]); // Bottom left
		let vertex6 = Vertex::new_with_texture(self.x+width, self.y+height,colour,[1.0,0.0]); //Bottom right
	
		return vec![vertex1,vertex2,vertex3,vertex4,vertex5,vertex6];
	}

	fn rasterize(&self,display:&Display<WindowSurface>) -> (Texture2d,Size) {
		let text_renderer = TextRenderer::default();
		let raw_image:RawImage2d<_>;
		let image_size:Size;

		// Render the text as a png
		let text_image = text_renderer.render_text_to_png_data(
			self.text.as_str(), 
			self.font_size, 
			self.colour.as_str()
		).unwrap();
		image_size = text_image.size;

		// Get the raw pixel values for the image
		let img = image::load(Cursor::new(text_image.data), image::ImageFormat::Png).unwrap().to_rgba8().into_raw();
		
		// Create an opengl raw image 
		raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&img,(image_size.width,image_size.height));

		// Create the texture from the image
		let texture = glium::texture::Texture2d::new(display, raw_image).unwrap();

		return (texture,image_size);
	}
}

impl Surface for TextSurface {
	fn draw(
		&mut self,
		display:&glium::Display<WindowSurface>,
		frame:&mut glium::Frame,
		window:&winit::window::Window,
		context:&RenderContext,
	) {

		let params = DrawParameters{
			blend:Blend::alpha_blending(),
			..Default::default()
		};

		let screen_width = window.inner_size().width as f32;
		let screen_height = window.inner_size().height as f32;

		self.build(display);

		let texture = self.texture.as_ref().expect("Null texture, call build before render");
		let size = self.size.as_ref().expect("Null size, call build before render");

		let uniforms = uniform! {
			width:screen_width,
			height:screen_height,
			tex: texture,
		};

		dbg!(texture,size);


		let vertices:Vec<Vertex> = self.to_vertices(size.width as i32, size.height as i32);
		let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap();
		let indices = index::NoIndices(glium::index::PrimitiveType::TrianglesList);
		
		frame.draw(
			&vertex_buffer, 
			&indices, 
			&context.text_program, 
			&uniforms,
			&params
		).unwrap();
		
	}

	fn size(&mut self,width:u32,height:u32) {
		self.width = width;
		self.height = height;
	}
	
	fn get_size(&self) -> (u32,u32) {
		(self.width,self.height)
	}

	fn get_bounds(&self) -> Bounds {
		Bounds{
			x:[self.x,self.x + self.width as i32],
			y:[self.y,self.y + self.height as i32]
		}
	}

	fn position(&mut self, x:i32,y:i32) {
		self.x = x;
		self.y = y;
	}

	fn get_position(&self) -> (i32,i32) {
		(self.x,self.y)
	}
}
