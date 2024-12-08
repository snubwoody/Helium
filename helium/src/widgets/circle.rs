use helium_core::color::Color;
use nanoid::nanoid;

use crate::{layout::IntrinsicSize, surface::circle::CircleSurface};

use super::{Widget, WidgetBody};


#[derive(Debug,Clone)]
pub struct Circle{
	radius:u32,
	color:Color
}

impl Circle {
	pub fn new(radius:u32,color:Color) -> Self{
		Self{ radius,color }
	}
}

impl Widget for Circle {
	fn build(&self) -> WidgetBody {
		let surface = CircleSurface::new(self.radius,self.color.clone());
		let intrinsic_size = IntrinsicSize::fixed(self.radius, self.radius);

		WidgetBody { 
			id: nanoid!(), 
			surface:Box::new(surface),
			intrinsic_size,
			..Default::default()
		}
	}
}