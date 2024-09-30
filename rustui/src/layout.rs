use crate::widgets::{WidgetBody, WidgetTree};

/// The types of layout a [`Widget`] can have.
#[derive(Debug,Clone, Copy)]
pub enum Layout{
	Horizontal{
		spacing:u32,
		padding:u32,
	},
	Vertical{
		spacing:u32,
		padding:u32,
	},
	Block{
		padding:u32,
	},
}

impl Layout {
	pub fn arrange_widgets(&self,widgets:&mut Vec<Box<WidgetBody>>){
		match self {
			Self::Horizontal { spacing, padding } => self.arrange_horizontal(widgets,*padding,*spacing),
			Self::Vertical { spacing, padding } => self.arrange_vertical(widgets,*padding,*spacing),
			Self::Block { padding } => self.arrange_block(widgets,*padding),
		}
	}

	fn arrange_horizontal(&self,widgets:&mut Vec<Box<WidgetBody>>,padding:u32,spacing:u32){
		// Set the initial position to the padding
		let mut current_pos = padding;

		for (_,widget) in widgets.iter_mut().enumerate(){
			widget.surface.position(current_pos as f32, padding as f32);
			
			// Add the spacing and the widget's width to the current
			// position
			current_pos += spacing;
			current_pos += widget.surface.get_size().width as u32;

			// Arrange the widget's children recursively
			widget.layout.arrange_widgets(&mut widget.children);
		}
	}

	fn arrange_vertical(&self,widgets:&mut Vec<Box<WidgetBody>>,padding:u32,spacing:u32){
		// Set the initial position to the padding
		let mut current_pos = padding;

		for (_,widget) in widgets.iter_mut().enumerate(){
			widget.surface.position(padding as f32, current_pos as f32);
			
			// Add the spacing and the widget's width to the current
			// position
			current_pos += spacing;
			current_pos += widget.surface.get_size().height as u32;

			// Arrange the widget's children recursively
			widget.layout.arrange_widgets(&mut widget.children);
		}
	}

	fn arrange_block(&self,widgets:&mut Vec<Box<WidgetBody>>,padding:u32){
		// Block elements should only have one child
		// so no need to iterate
		if widgets.is_empty() {
			return;
		}
		widgets[0].surface.position(padding as f32, padding as f32);
	}
}

#[derive(Debug,Clone, Copy)]
pub enum WidgetSize{
	Fixed(f32),
	Fill,
	Fit(f32)
}

impl WidgetSize {
	pub fn size_widgets(&self,constraint:f32,widgets:&mut Vec<Box<WidgetBody>> ) -> f32{
		match self {
			Self::Fill => constraint,
			Self::Fit(padding) => 0.0,
			Self::Fixed(size) => *size
		}
	}
}

#[derive(Debug,Clone,Copy)]
pub struct IntrinsicSize {
	pub width:WidgetSize,
	pub height:WidgetSize
}

/// The [`Widget`] constraints that are used when calculating
/// it's size.
#[derive(Debug,Clone,Copy,PartialEq, PartialOrd,Default)]
pub struct Constraint{
	pub max_width:f32,
	pub min_width:f32,
	pub max_height:f32,
	pub min_height:f32,
}

impl Constraint {
	pub fn new(max_width:f32,min_width:f32,max_height:f32,min_height:f32) -> Self{
		Self { max_width, min_width, max_height, min_height }
	}
}

