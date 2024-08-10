use crate::{
	colour::Colour, 
	surface::Surface, 
	widgets::WidgetBody,
	layout::Layout
};
use super::Widget;

#[derive(Debug)]
pub struct Button{
	pub text:String
}

impl Widget for Button {
	fn build(&self) -> WidgetBody {
		let surface = Surface::new(0, 0, 200, 70, Colour::Rgb(25, 125, 255));
		let layout = Layout::SingleChild { width: 250, height: 70 };
		WidgetBody { surface,layout,children:vec![],events:vec![] }
	}
}