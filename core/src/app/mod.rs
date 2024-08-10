use std::fs;
use glium::{
	backend::glutin::SimpleWindowBuilder,
	glutin::surface::WindowSurface, Display, Program,
};
use winit::{
	dpi::PhysicalPosition, 
	event::WindowEvent, 
	event_loop::{
		ControlFlow, 
		EventLoop
	}, 
	window::Window
};
use crate::{app::view::{RenderContext, View}, widgets::WidgetTree};
pub mod view;


/// This is a singular isolated program. Most projects
/// will only contain one app
pub struct App{
	event_loop:EventLoop<()>,
	window:Window,
	display:Display<WindowSurface>,
	views:Vec<View>,
	context:RenderContext,
	index:usize
}

impl App{
	pub fn new() -> Self {
		let event_loop = EventLoop::new().unwrap();

		// Set the control flow to redraw every frame whether
		// there are events to process or not
		event_loop.set_control_flow(ControlFlow::Poll);
		
		let (window,display) = SimpleWindowBuilder::new().build(&event_loop);

		// Compile the shaders
		let surface_program = create_program(&display,"core/shaders/surface.vert","core/shaders/surface.frag");
		let text_program = create_program(&display,"core/shaders/text.vert","core/shaders/text.frag");

		let context = RenderContext::new(surface_program, text_program);

		Self { event_loop,window,display,context,views:vec![],index:0}
	}

	pub fn add_view(mut self,view:View) -> Self{
		self.views.push(view);
		self
	}

	pub fn run(mut self){
		self.event_loop.run(move | event,window_target|{
			match event {
				winit::event::Event::WindowEvent{event,..} => match event {
					WindowEvent::CloseRequested => window_target.exit(),
					WindowEvent::RedrawRequested => {
						self.views[self.index].render(&self.display, &self.window,&self.context)
					},
					WindowEvent::CursorMoved { position,.. } => {
						handle_hover_event(&mut self.views[0].widget_tree,position);
					}
					_ => {}
				}, 
				_ => {}
			}
	
		}).expect("Event loop error occured");
	}
}

enum Event{
	OnHover,
	OnClick,
}


fn handle_hover_event(widget_tree:&mut WidgetTree,position:PhysicalPosition<f64>){
	for (_,widget) in  widget_tree.widgets.iter_mut().enumerate(){
		let bounds = widget.get_bounds();
		let cursor_pos = [position.x as i32,position.y as i32];
		if bounds.within(cursor_pos){
			widget.on_hover()
		}
	}
}
fn create_program(display:&Display<WindowSurface>,vertex_shader_src:&str,fragment_shader_src:&str) -> Program {
	let vertex_shader = fs::read_to_string(vertex_shader_src).expect("Cannot locate vertex shader file");
	let fragment_shader = fs::read_to_string(fragment_shader_src).expect("Cannot locate vertex shader file");
	let program = glium::Program::from_source(display, vertex_shader.as_str(), fragment_shader.as_str(), None).unwrap();
	return program
}