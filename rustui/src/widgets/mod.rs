pub mod rect;
pub mod stack;
pub mod container;
pub mod text;
pub mod button;
pub mod list;
pub mod image;
pub mod flex;
use std::{collections::HashMap, fmt::Debug, ops::Deref};
use glium::{
	glutin::surface::WindowSurface, Display, Frame, 
};
use winit::window::Window;
use crate::{
	app::{
		events::EventFunction,
		view::RenderContext
	}, layout::{Layout, SizeConstraint}, surface::{
		rect::RectSurface, Surface
	}
};


// TODO change size to floating point values
// TODO change render name to draw

/// Widget trait that all widgets must inherit from.
pub trait Widget:Debug{
	/// Build the [`Widget`] into a primitive [`WidgetBody`]
	fn build(&self) -> WidgetBody;

	/// Get the children and consume the [`Widget`], since this is 
	/// the last step before the widget is turned to a 
	/// [`WidgetBody`].
	fn get_children(self) -> Vec<Box<dyn Widget>>;
}

/// Primitive structure that holds all the information
/// about a [`Widget`] required for rendering.
pub struct WidgetBody{
	pub surface:Box<dyn Surface>,
	pub layout:Layout,
	pub children:Vec<Box<WidgetBody>>,
	pub constraint:SizeConstraint
	//pub events:Vec<EventFunction>
}

impl WidgetBody {
	/// Draw the [`WidgetBody`] to the screen.
	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		// Arrange the children
		self.arrange_widgets();

		// Render the parent then the children
		self.surface.draw(display, frame, window, context);
		self.children.iter_mut().for_each(|widget|widget.render(display, frame, window, context));
	}

	/// TODO
	pub fn arrange_widgets(&mut self) {
		// Arrange the children
		let position = self.surface.get_position();
		self.children.iter_mut().for_each(|widget| {
			widget.arrange_widgets();
		});

		let size = self.layout.arrange([position.0,position.1],&mut self.children);
		self.surface.size(size.0 as f32, size.1 as f32);
	}


}

impl Default for WidgetBody {
	fn default() -> Self {
		let surface = Box::new(RectSurface::default());
		let layout = Layout::Single{ padding: 0 };

		Self { 
			surface, 
			layout, 
			children:vec![], 
			constraint:SizeConstraint::Fit
			//events:vec![]
		}
	}
}

pub struct WidgetNode{
	pub body:WidgetBody,
	pub id:usize,
	pub parent:Option<usize>,
	pub children:Vec<usize>
}

// FIXME kind of unnecessary right not so maybe remove it
// TODO maybe implement iter for the widget tree

/// Central structure that holds all the [`Widget`]'s, this is 
/// where rendering, layouts and events are processed from.
pub struct WidgetTree{
	pub widgets:Vec<WidgetBody>,
}

impl WidgetTree where  {
	pub fn new() -> Self{
		Self { 
			widgets:Vec::new(),
		}
	}

	/// Add a [`WidgetNode`] to the tree
	pub fn add(
		&mut self,
		widget:impl Widget + 'static,
		id:usize,
		parent_id:Option<usize>,
		children:Vec<usize>
	) {

		self.widgets.push(widget.build());
	}


	fn walk(&self){
	
	}

	fn arrange_layouts(&self){
	}

	/// Draw the [`WidgetTree`] to the screen.
	pub fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		context:&RenderContext,
	) {
		self.widgets.iter_mut().for_each(|widget| {
			widget.render(display, frame, window, context);
		})
	}
}

