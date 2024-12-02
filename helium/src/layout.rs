use helium_core::{position::Position, size::Size};
use wgpu::rwh;
use crate::widgets::WidgetBody;

/// How a [`Widget`] should align it's children
#[derive(Debug,Clone, Copy,PartialEq,Eq,Default)]
pub enum AxisAlignment{
	#[default]
	Start,
	Center,
	End,
	/// The parent [`Widget`] will attempt to have as much spacing 
	/// between the widgets as possible.
	SpaceBetween,
	/// The parent [`Widget`] will attempt to have equal spacing 
	/// both around and between the widgets.
	SpaceEvenly
}

#[derive(Debug,Clone, Copy,PartialEq, Eq,Default)]
pub enum LayoutType {
	#[default]
	Block,
	Horizontal,
	Vertical,
}

/// Handles the layout of `widgets`. It works by calculating the max size
/// which is the maximum size that widget's are allowed to be and the min size
/// which is the mininum space required to fit a widgets children. The if a widget
/// is set to `fill` it will use the max size and if it is set to `fit` then it will
/// use the min size.
#[derive(Debug,Clone, Copy,Default)]
pub struct Layout{
	spacing:u32,
	padding:u32,
	layout:LayoutType,
	main_axis_alignment:AxisAlignment,
	cross_axis_alignment:AxisAlignment
}

impl Layout{
	pub fn new() -> Self{
		Self::default()
	}

	pub fn horizontal() -> Self{
		Self{
			layout:LayoutType::Horizontal,
			..Default::default()
		}
	}

	pub fn block() -> Self{
		Self{
			layout:LayoutType::Block,
			..Default::default()
		}
	}

	pub fn vertical() -> Self{
		Self{
			layout:LayoutType::Vertical,
			..Default::default()
		}
	}

	pub fn spacing(mut self,spacing:u32) -> Self{
		self.spacing = spacing;
		self
	}

	pub fn padding(mut self,padding:u32) -> Self{
		self.padding = padding;
		self
	}

	pub fn layout(mut self,layout:LayoutType) -> Self{
		self.layout = layout;
		self
	}

	pub fn main_axis_alignment(mut self,main_axis_alignment:AxisAlignment) -> Self{
		self.main_axis_alignment = main_axis_alignment;
		self
	}

	pub fn cross_axis_alignment(mut self,cross_axis_alignment:AxisAlignment) -> Self{
		self.cross_axis_alignment = cross_axis_alignment;
		self
	}

	
	/// Arrange and size the widgets.
	pub fn arrange_widgets(
		&self,
		widgets:&mut Vec<Box<WidgetBody>>,
		max_size:Size,
		parent_pos:Position
	) -> Size{
		match self.layout {
			LayoutType::Horizontal => 
				self.arrange_horizontal(widgets,max_size,parent_pos),
			LayoutType::Vertical => 
				self.arrange_vertical(widgets,max_size,parent_pos),
			LayoutType::Block => 
				self.arrange_block(widgets,&max_size,&parent_pos),
		}
	}

	/// Calculate the maximum [`Size`] that the [`Widget`]'s children are allowed to
	/// be. If multiple `widgets` are set to `fill` then the size will be 
	/// distributed among them. 
	fn max_size(&self,widgets:&[Box<WidgetBody>],max_size:Size) -> Size {
		// The maximum size for the widget children to be
		let mut size = max_size;

		// The number of widgets that have that their size set to fill
		let mut width_fill_count = 0;
		let mut height_fill_count = 0;

		size.width -= (self.padding * 2) as f32;
		size.height -= (self.padding * 2) as f32;
		
		for (i,widget) in widgets.iter().enumerate(){
			// Subtract the spacing for every element except the last
			if i != widgets.len() - 1{
				size.width -= self.spacing as f32; // TEMP
				size.height -= self.spacing as f32; // TEMP
			}

			// TODO maybe move this to the enum?
			match widget.intrinsic_size.width {
				WidgetSize::Fixed(width) => {
					size.width -= width
				}
				WidgetSize::Fit => {
					size.width += widget.surface.get_size().width;
				},
				WidgetSize::Fill => {
					width_fill_count += 1;
				}
			}

			match widget.intrinsic_size.height {
				WidgetSize::Fixed(height) => {
					size.height -= height
				}
				WidgetSize::Fit => {
					size.height += widget.surface.get_size().height;
				},
				WidgetSize::Fill => {
					height_fill_count += 1;
				}
			}
		};

		// Distribute the size evenly among the children 
		size.width /= width_fill_count as f32;
		size.height /= height_fill_count as f32;

		size
	}

	/// Position the `Widgets` according to the [`AxisAlignment`]
	fn align(&self,widgets:&mut Vec<Box<WidgetBody>>,parent_pos:&Position){
		let mut current_pos = self.padding as f32 + parent_pos.x;

		for widget in widgets{
			// Set the current widget position
			match self.layout {
				LayoutType::Vertical => {
					widget.surface.position(parent_pos.y + self.padding as f32,current_pos);
					// Add the spacing and the widget's width to the current
					// position and the min width
					current_pos += self.spacing as f32;
					current_pos += widget.surface.get_size().height;
				},
				LayoutType::Horizontal => {
					widget.surface.position(current_pos as f32, parent_pos.y + self.padding as f32);
					// Add the spacing and the widget's width to the current
					// position and the min width
					current_pos += self.spacing as f32;
					current_pos += widget.surface.get_size().width;
				}
				LayoutType::Block => {}
			}
			
		}
	}

	fn align_vertical(&self){

	}

	fn arrange_horizontal(
		&self,
		widgets:&mut Vec<Box<WidgetBody>>,
		max_size:Size,
		parent_pos:Position
	) -> Size {
		let mut min_width:f32 = 0.0;
		let mut min_height:f32 = 0.0;

		if widgets.is_empty(){
			return Size::default()
		}

		// Currently max size only affects widgets will fill sizing
		// widgets with fit use min width and fixed ignores everything
		let child_max_size = self.max_size(widgets, max_size);

		for widget in widgets.iter_mut(){
			// Arrange the widget's children recursively and return the minimum 
			// size required occupy all the children.
			let size = widget.layout.arrange_widgets(
				&mut widget.children,
				max_size,
				widget.surface.get_position()
			);

			// Set the widget's size
			match widget.intrinsic_size.width {
				WidgetSize::Fill => widget.surface.width(child_max_size.width),
				WidgetSize::Fit => widget.surface.width(size.width),
				WidgetSize::Fixed(size) => widget.surface.width(size),
			}

			match widget.intrinsic_size.height {
				WidgetSize::Fill => widget.surface.height(max_size.height),
				WidgetSize::Fit => widget.surface.height(size.height),
				WidgetSize::Fixed(size) => widget.surface.height(size),
			}

			min_width += self.spacing as f32;
			min_width += widget.surface.get_size().width;

			// Set the minimum height to the height of the largest widget
			min_height = min_height.max(widget.surface.get_size().height);
		};

		self.align(widgets, &parent_pos);
		min_width += (self.padding * 2) as f32;
		min_height += (self.padding * 2) as f32;
		Size::new(min_width, min_height)
	}

	fn arrange_vertical(
		&self,
		widgets:&mut Vec<Box<WidgetBody>>,
		max_size:Size,
		parent_pos:Position
	) -> Size{
		// Set the initial position to the padding plus 
		// the parent position
		let mut min_width:f32 = 0.0;
		let mut min_height:f32 = 0.0;

		if widgets.is_empty(){
			return Size::default()
		}


		let child_max_size = self.max_size(widgets, max_size);

		widgets.iter_mut().for_each(|widget|{
			// Arrange the widget's children recursively and return the min size
			let size = widget.layout.arrange_widgets(
				&mut widget.children,
				max_size,
				widget.surface.get_position()
			);

			// Set the widget's size
			match widget.intrinsic_size.width {
				WidgetSize::Fill => widget.surface.width(child_max_size.width),
				WidgetSize::Fit => widget.surface.width(size.width),
				WidgetSize::Fixed(size) => widget.surface.width(size),
			}

			match widget.intrinsic_size.height {
				WidgetSize::Fill => widget.surface.height(child_max_size.height),
				WidgetSize::Fit => widget.surface.height(size.height),
				WidgetSize::Fixed(size) => widget.surface.height(size),
			}

			min_height += self.spacing as f32;
			min_height += widget.surface.get_size().height;

			// Set the minimum width to the width of the largest widget
			min_width = min_width.max(widget.surface.get_size().width);
		});

		self.align(widgets, &parent_pos);
		min_width += (self.padding * 2) as f32;
		min_height += (self.padding * 2) as f32;
		
		Size::new(min_width,min_height)
	}

	fn arrange_block(
		&self,
		widgets:&mut Vec<Box<WidgetBody>>,
		max_size:&Size,
		parent_pos:&Position
	) -> Size{
		// Return if element has no children
		if widgets.is_empty() {
			return  Size::default()
		}

		let mut min_width = self.padding as f32 * 2.0;
		let mut min_height = self.padding as f32 * 2.0;

		widgets.iter_mut().for_each(|widget|{
			widget.surface.position(
				parent_pos.x + self.padding as f32, 
				parent_pos.y + self.padding as f32
			);

			// If the widget has no children return
			// it's size 
			let size = if widget.children.is_empty() {
				widget.surface.get_size()
			} else{
				widget.layout.arrange_widgets(
					&mut widget.children, 
					*max_size,
					Position::new(
						widget.surface.get_position().x,
						widget.surface.get_position().y
					)
				)
			};


			min_width += size.width;
			min_height += size.height;
			
			// Set the widget's size
			match widget.intrinsic_size.width {
				WidgetSize::Fill => widget.surface.width(max_size.width - self.padding as f32),
				WidgetSize::Fit => widget.surface.width(size.width),
				WidgetSize::Fixed(size) => widget.surface.width(size),
			}

			match widget.intrinsic_size.height {
				WidgetSize::Fill => widget.surface.height(max_size.height),
				WidgetSize::Fit => widget.surface.height(size.height),
				WidgetSize::Fixed(size) => widget.surface.height(size),
			}
		});

		
		Size::new(min_width, min_height)
	}
}


#[derive(Debug,Clone, Copy,Default,PartialEq)]
pub enum WidgetSize{
	Fixed(f32),
	Fill,
	#[default]
	Fit,
}


/// This is the size that a [`Widget`] will try to be,  
/// the actual final size is dependant on the space
/// available.
#[derive(Debug,Clone,Copy,Default)]
pub struct IntrinsicSize {
	pub width:WidgetSize,
	pub height:WidgetSize
}

impl IntrinsicSize {
	pub fn fixed(width:u32,height:u32) -> Self{
		Self { 
			width: WidgetSize::Fixed(width as f32), 
			height: WidgetSize::Fixed(height as f32) 
		}
	}
	
	pub fn fit() -> Self {
		IntrinsicSize{
			width:WidgetSize::Fit,
			height:WidgetSize::Fit,
		}
	}

	pub fn fill(&mut self){
		self.width = WidgetSize::Fill;
		self.height = WidgetSize::Fill;
	}

	pub fn fill_width(&mut self){
		self.width = WidgetSize::Fill;
	}

	pub fn fill_height(&mut self){
		self.height = WidgetSize::Fill;
	}
}

#[cfg(test)]
mod test{
	use crate::{hstack, widgets::{Rect, Widget}};
	use super::*;

	#[test]
	fn test_horizontal_fit_size(){
		let spacing = 12;
		let padding = 12;
		let window = Size::new(800.0, 800.0);
		let mut empty_box = WidgetBody::new(); // TODO test the other layouts
		empty_box.layout.spacing(spacing);
		empty_box.layout.padding(padding);
		empty_box.arrange(window);

		assert_eq!(empty_box.surface.get_size(),Size::new(0.0,0.0));

		let box1 = WidgetBody::new().intrinsic_size(IntrinsicSize::fixed(200, 200));
		let box2 = WidgetBody::new().intrinsic_size(IntrinsicSize::fixed(200, 200));
		
		let mut horizontal_box = WidgetBody::new()
			.layout(
				Layout::horizontal().spacing(spacing).padding(padding)
			)
			.add_children(vec![box1,box2])
			.intrinsic_size(IntrinsicSize::fit());
		
		
		horizontal_box.arrange(window);
		let mut expected_size = Size::new(400.0, 200.0);
		expected_size.width += (padding * 2) as f32;
		expected_size.width += (spacing * 2) as f32;
		expected_size.height += (padding * 2) as f32;

		
	}

	#[test]
	fn test_vertical_fit_size(){
		let spacing = 12;
		let padding = 12;
		let window = Size::new(800.0, 800.0);
		// The width should be equal to the sum of the width of both widgets plus 
		// the padding plus the spacing in between widgets 
		
		
		let box1 = WidgetBody::new().intrinsic_size(IntrinsicSize::fixed(200, 200));
		let box2 = WidgetBody::new().intrinsic_size(IntrinsicSize::fixed(200, 200));
		
		let mut vertical_box = WidgetBody::new()
			.layout(
				Layout::vertical().spacing(spacing).padding(padding)
			)
			.add_children(vec![box1,box2])
			.intrinsic_size(IntrinsicSize::fit());

		vertical_box.arrange(window);
		
		let mut expected_size = Size::new(200.0, 400.0);
		expected_size.height += (padding * 2) as f32;
		expected_size.height += (spacing * 2) as f32;
		expected_size.width += (padding * 2) as f32;
		assert_eq!(vertical_box.surface.get_size(),expected_size);
	}

	#[test]
	fn test_fill_size(){

	}

	#[test]
	fn test_vertical_layout(){
		
	}

	#[test]
	fn test_block_layout(){
		
	}

	#[test]
	fn test_alignment(){
		
	}

	#[test]
	fn test_sizing(){
		
	}
}


