use nanoid::nanoid;
use crate::{
	app::events::{EventSignal, EventType}, 
	layout::{IntrinsicSize, Layout, WidgetSize}, 
	surface::rect::RectSurface, 
	widgets::WidgetBody
};
use helium_core::color::Color;
use super::{text::Text, Widget, WidgetId};
use crate::app::events::Event;

/// A simple button.
pub struct Button{
	id:String,
	text:String,
	color:Color,
	padding:u32,
	width:WidgetSize,
	height:WidgetSize,
}

impl Button {
	pub fn new(text:&str) -> Self {
		Self { 
			id:nanoid!(),
			text:text.into(), 
			color:Color::Rgb(255, 255, 255),
			padding:12,
			width: WidgetSize::Fit,
			height:WidgetSize::Fit,
		}
	}

	pub fn get_id(&self) -> WidgetId{
		self.id.clone()
	}

	pub fn color(mut self,color:Color) -> Self {
		self.color = color;
		self
	}

	pub fn padding(mut self, padding:u32) -> Self {
		self.padding = padding;
		self
	}

	pub fn width(mut self, width:f32) -> Self{
		self.width = WidgetSize::Fixed(width);
		self
	}

	pub fn height(mut self, height:f32) -> Self{
		self.height = WidgetSize::Fixed(height);
		self
	}

	pub fn fill(mut self) -> Self{
		self.width = WidgetSize::Fill;
		self
	}
}

// FIXME button text not working
impl Widget for Button {
	fn build(&self) -> WidgetBody {
		let surface = Box::new(
			RectSurface::new(0.0, 0.0, 200.0, 70.0, self.color.clone())
		);

		let layout = Layout::new().padding(self.padding);

		let text_body = Box::new(Text::new(&self.text)).build();

		let intrinsic_size = IntrinsicSize{
			width:self.width,
			height:self.height
		};

		WidgetBody { 
			id:self.id.clone(),
			surface,
			layout,
			intrinsic_size,
			children: vec![
				Box::new(text_body)
			],
			..Default::default()
		}
	}
}