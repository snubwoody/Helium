use nanoid::nanoid;

use crate::{
	app::events::{Event, }, layout::{IntrinsicSize, Layout, WidgetSize}, surface::{text::TextSurface, Surface} 
};
use super::{Widget, WidgetBody};

pub struct Text{
	id:String,
	text:String,
	font_size:u8,
}

impl Text {
	pub fn new(text:&str) -> Self{
		Self { 
			id:nanoid!(),
			text:text.into(), 
			font_size:16,
		}	
	}

	/// Set the font size
	pub fn font_size(mut self,size:u8) -> Self{
		self.font_size = size;
		self
	}
}

impl Widget for Text {
	fn build(&self) -> WidgetBody {
		// Create the text surface to be rendered
		let textsurface = TextSurface::new(
			self.text.as_str(),
			"#000000" , 
			self.font_size
		);

		let size = textsurface.get_size();
		let surface = Box::new(textsurface);

		let intrinsic_size = IntrinsicSize{
			width:WidgetSize::Fixed(size.width),
			height:WidgetSize::Fixed(size.height)
		};

		WidgetBody{
			id:self.id.clone(),
			surface,
			intrinsic_size,
			..Default::default()
		}
	}
}
