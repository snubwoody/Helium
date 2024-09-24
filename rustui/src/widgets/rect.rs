use crate::surface::rect::RectSurface;
use crate::utils::Size;
use crate::{colour::Colour};
use super::{Widget, WidgetBody};
use crate::layout::{IntrinsicSize, Layout};

/// A simple rectangle
#[derive(Debug,Clone,PartialEq, Eq)]
pub struct Rect{
	pub width:u32,
	pub height:u32,
	pub colour:Colour
}

impl Rect {
	pub fn new(width:u32,height:u32,colour:Colour) -> Self{
		Self { width, height, colour }
	}
}

impl Widget for Rect {
	fn build(&self) -> WidgetBody {
		let layout = Layout::Block { padding: 0 };
		let surface = Box::new(
			RectSurface{ 
				size:Size::new(self.width as f32, self.height as f32),
				colour:self.colour.clone(),
				..Default::default()
			}
		);
		
		WidgetBody{ 
			surface,
			layout,
			children:vec![],
			constraint:IntrinsicSize::Fixed(self.width as f32, self.height as f32),
			..Default::default()
		}
	}

	fn get_children(self) -> Vec<Box<dyn Widget>> {
		vec![]
	}
}
 