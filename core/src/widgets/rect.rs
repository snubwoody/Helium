use crate::{colour::Colour};
use crate::Surface;
use super::{Widget, WidgetBody};
use crate::layout::Layout;

/// A simple rectangle
#[derive(Debug,Clone,Copy)]
pub struct Rect{
	pub width:u32,
	pub height:u32,
	pub colour:Colour
}

impl Widget for Rect {
	fn build(&self) -> WidgetBody {
		let layout = Layout::Single { padding: 0 };
		WidgetBody{ 
			surface:Surface{ 
				x:0, 
				y:0, 
				width:self.width as i32,
				height:self.height as i32,
				colour:self.colour
			},
			layout,
			children:vec![]
		}
	}
}
