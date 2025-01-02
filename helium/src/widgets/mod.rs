mod rect;
mod container;
mod text;
mod button;
mod circle;
mod vstack;
mod hstack;
mod spacer;
pub mod icon;
pub (crate) mod image;
use nanoid::nanoid;
pub use rect::*;
pub use text::*;
pub use button::*;
pub use hstack::*;
pub use vstack::*;
pub use container::*;
pub use circle::*;
pub use image::*;
pub use spacer::*;
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
#[macro_export]
macro_rules! impl_widget {
	() => {
		pub fn fill(mut self) -> Self{
			self.layout.intrinsic_size.width = crystal::BoxSizing::Flex(1);
			self.layout.intrinsic_size.height = crystal::BoxSizing::Flex(1);
			self
		}

		pub fn flex(mut self,factor:u8) -> Self{
			self.layout.intrinsic_size.width = crystal::BoxSizing::Flex(factor);
			self.layout.intrinsic_size.height = crystal::BoxSizing::Flex(factor);
			self
		}
	
		pub fn fit(mut self) -> Self{
			self.layout.intrinsic_size.width = crystal::BoxSizing::Shrink;
			self.layout.intrinsic_size.height = crystal::BoxSizing::Shrink;
			self
		}

		pub fn fill_width(mut self) -> Self{
			self.layout.intrinsic_size.width = crystal::BoxSizing::Flex(1);
			self
		}
	
		pub fn fill_height(mut self) -> Self{
			self.layout.intrinsic_size.height = crystal::BoxSizing::Flex(1);
			self
		}

		pub fn fixed_width(mut self,width:f32) -> Self{
			self.layout.intrinsic_size.width = crystal::BoxSizing::Fixed(width);
			self
		}
	
		pub fn fixed_height(mut self,height:f32) -> Self{
			self.layout.intrinsic_size.height = crystal::BoxSizing::Fixed(height);
			self
		}

		pub fn fit_width(mut self) -> Self{
			self.layout.intrinsic_size.width = crystal::BoxSizing::Shrink;
			self
		}
	
		pub fn fit_height(mut self) -> Self{
			self.layout.intrinsic_size.height = crystal::BoxSizing::Shrink;
			self
		}
	
		pub fn flex_width(mut self,factor:u8) -> Self{
			self.layout.intrinsic_size.height = crystal::BoxSizing::Flex(factor);
			self
		}
		
		pub fn flex_height(mut self,factor:u8) -> Self{
			self.layout.intrinsic_size.height = crystal::BoxSizing::Flex(factor);
			self
		}
	};
}