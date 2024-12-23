mod rect;
mod container;
mod text;
mod button;
mod circle;
mod vstack;
mod hstack;
use nanoid::nanoid;
pub use rect::Rect;
pub use text::Text;
pub use button::Button;
pub use hstack::HStack;
pub use vstack::VStack;
pub use container::Container;
pub use circle::Circle;
use crate::{
	app::AppState, 
	surface::{
		rect::RectSurface, Surface
	}, 
};
use helium_core::position::Position;
use helium_core::size::Size;
use crystal::{EmptyLayout, Layout, LayoutSolver};

pub type WidgetId = String; // FIXME Redundant type

/// The trait that all widgets must implement. Each `widget` must implement build function
/// which returns a [`WidgetBody`]. `widgetbodies` are objects that hold information about 
/// the widget.
pub trait Widget{
	// I've changed this between &self and self, a couple times and my conclusion is 
	// just keep it as &self forever, it makes it way easier to compose multiple sub-widgets.

	/// Build the [`Widget`] into a primitive [`WidgetBody`] for
	/// rendering.
	fn build(&self) -> WidgetBody;
}

/// Primitive structure that holds all the information
/// about a [`Widget`] required for rendering.
pub struct WidgetBody{ // TODO this changes a lot so make these fields private
	pub id:WidgetId,
	/// A label for debugging purposes
	pub label:Option<String>,
	pub surface:Box<dyn Surface>,
	pub layout:Box<dyn crystal::Layout>,
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

	pub fn layout(mut self,layout:impl Layout + 'static) -> Self{
		self.layout = Box::new(layout);
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

	/// Draw the [`WidgetBody`] to the screen.
	pub(crate) fn render(
		&mut self,
		render_pass:&mut wgpu::RenderPass,
		state: &AppState
	) {
		let context = &state.context;

		LayoutSolver::solve(&mut *self.layout,state.size);
		
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
			layout:Box::new(EmptyLayout::new()), 
			children:vec![], 
		}
	}
}


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