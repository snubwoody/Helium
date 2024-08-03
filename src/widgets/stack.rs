use glium::{
	glutin::surface::WindowSurface, Display, Frame,  
};
use winit::window::Window;
use crate::{colour::rgb, surface::Surface, view::RenderContext, widgets::Widget};
use crate::layout::{Horizontal, Layout, Vertical};

pub struct VStack{
	surface:Surface,
	layout:Layout<Vertical>,
	children:Vec<Box<dyn Widget>>
}

impl VStack {
	pub fn new(spacing:u32,children:Vec<Box<dyn Widget>>) -> Self{
		let surface = Surface::new(0, 0, 0, 0, rgb(255, 255, 255));
		let layout = Layout::new(spacing, 120, Vertical);

		Self { surface, children,layout }
	}

	pub fn colour(mut self,colour:[f32;4]) -> Self{
		self.surface.colour = colour;
		self
	}
	
	pub fn arrange_widgets(&mut self){
		let (x,y) = (self.surface.x as u32,self.surface.y as u32);
		let (max_width,max_height) = self.layout.arrange([x,y], &mut self.children);
		self.size(max_width,max_height);
	}

}

impl Widget for VStack {
	fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		let position = [self.surface.x as u32,self.surface.y as u32];
		let (width,height) = self.layout.arrange(position, &mut self.children);
		self.size(width, height);

		self.surface.render(display, frame, window, &context.surface_program);
		self.children.iter_mut().for_each(|child|{
			child.render(display, frame, window, context)
		})
	}

	fn position(&mut self,x:i32,y:i32) {
		self.surface.x = x;
		self.surface.y = y;
	}

	fn size(&mut self,width:u32,height:u32) {
		self.surface.width = width as i32;	
		self.surface.height = height as i32;	
	}

	fn get_size(&self) -> (u32,u32) {
		(self.surface.width as u32,self.surface.height as u32)
	}

}

pub struct HStack{
	surface:Surface,
	layout:Layout<Horizontal>,
	children:Vec<Box<dyn Widget>>
}

impl HStack {
	pub fn new(spacing:u32,children:Vec<Box<dyn Widget>>) -> Self{
		let surface = Surface::new(0, 0, 0, 0, rgb(255, 255, 255));
		let layout = Layout::new(spacing, 0, Horizontal);
		Self { surface,layout,children }
	}

	pub fn colour(mut self,colour:[f32;4]) -> Self{
		self.surface.colour = colour;
		self
	}

	pub fn arrange_widgets(&mut self) {
		let (x,y) = (self.surface.x as u32,self.surface.y as u32);
		let (max_width,max_height) = self.layout.arrange([x,y], &mut self.children);
		self.size(max_width,max_height);
	}
}

impl Widget for HStack {
	fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		self.arrange_widgets();
		self.surface.render(display, frame, window, &context.surface_program);
		self.children.iter_mut().for_each(|child|{
			child.render(display, frame, window, context)
		})
	}

	fn position(&mut self,x:i32,y:i32) {
		self.surface.x = x;
		self.surface.y = y;
	}

	fn size(&mut self,width:u32,height:u32) {
		self.surface.width = width as i32;
		self.surface.height = height as i32;
	}

	fn get_size(&self) -> (u32,u32) {
		(self.surface.width as u32,self.surface.height as u32)
	}
}
