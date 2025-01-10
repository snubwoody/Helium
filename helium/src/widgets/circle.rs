use super::Widget;
use crate::surface::Primitive;
use crystal::{BoxSizing, EmptyLayout, Layout};
use helium_core::color::Color;

#[derive(Debug, Clone)]
pub struct Circle {
    id: String,
    diameter: u32,
    color: Color,
}

impl Circle {
    pub fn new(diameter: u32, color: Color) -> Self {
        Self {
            id: nanoid::nanoid!(),
            diameter,
            color,
        }
    }
}

impl Widget for Circle {
	fn layout(&self) -> Box<dyn Layout> {
		let mut layout = EmptyLayout::new();
        layout.intrinsic_size.width = BoxSizing::Fixed(self.diameter as f32);
        layout.intrinsic_size.height = BoxSizing::Fixed(self.diameter as f32);
        layout.id = self.id.clone();
		
		Box::new(layout)
	}

	fn primitive(&self) -> Primitive {
		Primitive::Circle{ id: self.id.clone(), color: self.color }	
	}
}
