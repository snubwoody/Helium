mod rect;
mod container;
mod text;
mod button;
mod circle;
mod vstack;
mod hstack;
mod icon;
mod image;
use nanoid::nanoid;
pub use rect::Rect;
pub use text::Text;
pub use button::Button;
pub use hstack::HStack;
pub use vstack::VStack;
pub use container::Container;
pub use circle::Circle;
pub use image::*;
use crate::{
	app::AppState, 
	surface::{
		rect::RectSurface, Surface
	}, 
};
use crystal::Layout;
// TODO maybe test widgets with layouts to make sure everything's integrated properly;

/// The trait that all widgets must implement. Each `widget` must implement build function
/// which returns a [`WidgetBody`]. `widgetbodies` are objects that hold information about 
/// the widget.
pub trait Widget{
	// I've changed this between &self and self, a couple times and my conclusion is 
	// just keep it as &self forever, it makes it way easier to compose multiple sub-widgets.

	/// Build the [`Widget`] into a primitive [`WidgetBody`] for
	/// rendering.
	fn build(&self) -> (WidgetBody,Box<dyn Layout>);
}

/// Primitive structure that holds all the information
/// about a [`Widget`] required for rendering.
#[derive(Debug)]
pub struct WidgetBody{ // TODO this changes a lot so make these fields private
	pub id:String,
	/// A label for debugging purposes
	pub label:Option<String>,
	pub surface:Box<dyn Surface>,
	pub children:Vec<Box<WidgetBody>>,
}

impl WidgetBody {
	pub fn new() -> Self{
		Self::default()	
	}

	pub fn label(mut self,label:&str) -> Self {
		self.label = Some(label.to_owned());
		self
	}

	pub fn surface(mut self,surface:Box<dyn Surface>) -> Self{
		self.surface = surface;
		self
	}

	pub fn add_child(mut self,child:WidgetBody) -> Self{
		self.children.push(Box::new(child));
		self
	}

	pub fn add_children(mut self,children:Vec<WidgetBody>) -> Self{
		for child in children{
			self.children.push(Box::new(child));
		};
		self
	}

	fn check_size(&mut self,layout:Box<&dyn Layout>){
		if layout.id() == self.id{
			self.surface.size(
				layout.size().width, 
				layout.size().height
			);
			self.surface.position(
				layout.position().x, 
				layout.position().y
			);
			// println!(
			// 	"\nHit!!!\nSurface: {:?}",
			// 	self.surface,
			// );
		}
	}

	pub fn update_sizes(&mut self,root_layout:&Box<dyn Layout>){
		// FIXME this probably has disgusting performance
		for (_,layout) in root_layout.iter().enumerate(){
			self.check_size(layout);
		}
		for child in &mut self.children{
			child.update_sizes(root_layout);
		}
	}

	/// Draw the [`WidgetBody`] to the screen.
	pub fn render(
		&mut self,
		render_pass:&mut wgpu::RenderPass,
		state: &AppState
	) {
		let context = &state.context;

		// Draw the parent then the children to the screen
		self.surface.draw(render_pass, context,state);
		self.children.iter_mut().for_each(|child|{
			child.render(render_pass, state);
		});
	}
}

impl Default for WidgetBody {
	fn default() -> Self {
		let surface = Box::new(RectSurface::default());

		Self { 
			id:nanoid!(),
			surface, 
			label:None,
			children:vec![], 
		}
	}
}


// TODO remove this and replace with modifiers 
/// Implement common styling attributes
#[macro_export]
macro_rules! impl_style {
	() => {
		/// Change the [`Color`] of a [`Widget`].
		pub fn color(mut self,color:crate::Color) -> Self{
			self.color = color;
			self
		} 
	};
}

/// Implement common methods for widgets
/// TODO match arms for padding and spacing
#[macro_export]
macro_rules! impl_widget {
	(padding) => {
		pub fn padding(mut self, padding: u32) -> Self {
			self.layout.padding = padding;
			self
		}

		pub fn width_fit(mut self) -> Self{
			self.layout.intrinsic_size.width = BoxSizing::Shrink;
			self
		}
	
		pub fn width_fill(mut self) -> Self{
			self.layout.intrinsic_size.width = BoxSizing::Flex(1);
			self
		}
	
		pub fn width_flex(mut self,factor:u8) -> Self{
			self.layout.intrinsic_size.width = BoxSizing::Flex(factor);
			self
		}
	};
	(padding,spacing) => {
		pub fn padding(mut self, padding: u32) -> Self {
			self.layout.padding = padding;
			self
		}

		pub fn spacing(mut self, spacing: u32) -> Self {
			self.layout.spacing = spacing;
			self
		}

		pub fn width_fit(mut self) -> Self{
			self.layout.intrinsic_size.width = BoxSizing::Shrink;
			self
		}
	
		pub fn width_fill(mut self) -> Self{
			self.layout.intrinsic_size.width = BoxSizing::Flex(1);
			self
		}
	
		pub fn width_flex(mut self,factor:u8) -> Self{
			self.layout.intrinsic_size.width = BoxSizing::Flex(factor);
			self
		}
	};
	() => {
		pub fn width_fit(mut self) -> Self{
			self.layout.intrinsic_size.width = BoxSizing::Shrink;
			self
		}
	
		pub fn width_fill(mut self) -> Self{
			self.layout.intrinsic_size.width = BoxSizing::Flex(1);
			self
		}
	
		pub fn width_flex(mut self,factor:u8) -> Self{
			self.layout.intrinsic_size.width = BoxSizing::Flex(factor);
			self
		}
	};
}