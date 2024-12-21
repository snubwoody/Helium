use helium_core::{position::Position, size::Size};
use crate::widgets::WidgetBody;
use super::{AxisAlignment,Layout,WidgetSize};

#[derive(Debug,Clone,Copy)]
pub struct HorizontalLayout{
	spacing:u32,
	padding:u32,
	main_axis_alignment:AxisAlignment,
	cross_axis_alignment:AxisAlignment
}


impl HorizontalLayout {
	pub fn new(spacing:u32,padding:u32) -> Self{
		Self { 
			spacing,
			padding, 
			main_axis_alignment: AxisAlignment::Start, 
			cross_axis_alignment: AxisAlignment::Start 
		}
	}

	pub fn spacing(&mut self,spacing:u32){
		self.spacing = spacing;
	}

	pub fn padding(&mut self,padding:u32){
		self.padding = padding;
	}

	/// Calculates the maximum size of all the `fixed` widgets.
	fn fixed_size_sum(&self,widgets:&[Box<WidgetBody>]) -> Size {
		let mut sum = Size::default();

		for widget in widgets{
			match widget.intrinsic_size.width {
				WidgetSize::Fixed(width) => sum.width += width,
				_ => {}
			}

			match widget.intrinsic_size.height {
				WidgetSize::Fixed(height) => sum.height += height,
				_ => {}
			}
		}

		sum
	}
}


impl Layout for HorizontalLayout {
	fn compute_layout(
		&self,
		widgets:&mut Vec<Box<WidgetBody>>,
		available_space:Size,
		parent_pos:Position
	) -> Size {
		let mut min_width:f32 = 0.0;
		let mut min_height:f32 = 0.0;
		let length = widgets.len();

		if widgets.is_empty(){
			return Size::default()
		}

		// Currently max size only affects widgets will fill sizing
		// widgets with fit use min width and fixed ignores everything
		let child_max_size = self.available_space(widgets, available_space);

		for (i,widget) in widgets.iter_mut().enumerate(){
			// Arrange the widget's children recursively and return the minimum 
			// size required to occupy all the children.
			let size = widget.layout.compute_layout(
				&mut widget.children,
				child_max_size,
				widget.surface.get_position()
			);

			// Set the widget's size
			match widget.intrinsic_size.width {
				WidgetSize::Fill => widget.surface.width(child_max_size.width),
				WidgetSize::Fit => widget.surface.width(size.width),
				WidgetSize::Fixed(size) => widget.surface.width(size),
			}

			match widget.intrinsic_size.height {
				WidgetSize::Fill => widget.surface.height(available_space.height),
				WidgetSize::Fit => widget.surface.height(size.height),
				WidgetSize::Fixed(size) => widget.surface.height(size),
			}

			if i != length - 1{
				min_width += self.spacing as f32;
			}

			min_width += widget.surface.get_size().width;

			// Set the minimum height to the height of the largest widget
			min_height = min_height.max(widget.surface.get_size().height);
		};

		self.align(widgets, &parent_pos);
		
		min_width += (self.padding * 2) as f32;
		min_height += (self.padding * 2) as f32;

		Size::new(min_width, min_height)
	}

	fn available_space(&self,widgets:&[Box<WidgetBody>],available_space:Size) -> Size {
		// The maximum size for the widget children to be
		let mut size = available_space;

		// The number of widgets that have that their size set to fill
		let mut width_fill_count = 0;
		let mut height_fill_count = 0;

		size.width -= (self.padding * 2) as f32;
		size.height -= (self.padding * 2) as f32;

		// Subtract the total fixed size from the available space
		size = size - self.fixed_size_sum(widgets);
		
		for (i,widget) in widgets.iter().enumerate(){
			// Subtract the spacing for every element except the last
			if i != widgets.len() - 1{
				size.width -= self.spacing as f32;
				size.height -= self.spacing as f32;
			}

			// TODO maybe move this to the enum?
			match widget.intrinsic_size.width {
				WidgetSize::Fill => {
					width_fill_count += 1;
				},
				WidgetSize::Fit => {
					size.width -= widget.surface.get_size().width;
				}, 
				_ => {}				
			}

			match widget.intrinsic_size.height {
				WidgetSize::Fill => {
					height_fill_count += 1;
				},
				WidgetSize::Fit => {
					size.height -= widget.surface.get_size().height;
				},
				_ => {}
			}
		};

		// TODO could probably just multiply by a fraction instead
		// Distribute the size evenly among the children 
		if width_fill_count > 0 {
			size.width /= width_fill_count as f32;
		}
		if height_fill_count > 0{

			size.height /= height_fill_count as f32;
		}

		size
	}

	/// Position the `Widgets` according to the [`AxisAlignment`]
	fn align(&self,widgets:&mut Vec<Box<WidgetBody>>,parent_pos:&Position){
		// TODO i might be able to make this a provided method if i pass the spacing?
		let mut pos = parent_pos.clone();
		// Add the padding
		pos += self.padding as f32;

		for widget in widgets{
			// Set the current widget position
			widget.surface.position(pos.x,pos.y);

			// Add the spacing and the widget's width to the current
			// position and the min width
			pos.x += self.spacing as f32;
			pos.x += widget.surface.get_size().width;
			self.align(&mut widget.children, &widget.surface.get_position());
		}
	}
}		

#[cfg(test)]
mod test{
	use crate::layout::IntrinsicSize;
	use super::*;
	
	#[test]
	fn test_nested_positioning(){
		let spacing = 24;
		let padding = 24;
		let window = Size::new(500.0, 500.0);

		let inner_box1 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(150, 150));
		let inner_box2 = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(150, 150));
		
		let inner_hbox = WidgetBody::new()
		.layout(HorizontalLayout::new(spacing, padding))
		.add_children(vec![inner_box1,inner_box2]);
	
		let inner_box = WidgetBody::new().intrinsic_size(IntrinsicSize::new().fixed(20, 50));
		let mut hbox = WidgetBody::new()
			.layout(HorizontalLayout::new(spacing, padding))
			.add_children(vec![inner_box,inner_hbox]);

		hbox.arrange(window);

		let child = &hbox.children[1].children[1];

		let mut pos = hbox.surface.get_position();
		// Add the inner_box width
		pos.x += 20.0;
		// Add the spacing and the padding
		pos.x += (spacing * 2 + padding * 2) as f32;
		// Add only the passing to y
		pos.y += (padding * 2) as f32;
		// Add the width of the inner_box1
		pos.x += 150.0;
		
		assert_eq!(child.surface.get_position(),pos)
	}

	#[test]
	fn test_fill_sizing(){
		let spacing = 24;
		let padding = 56;
		let window = Size::new(500.0, 500.0);

		let intrinsic_size = IntrinsicSize{
			width:WidgetSize::Fill,
			height:WidgetSize::Fill
		};

		let mut hbox = WidgetBody::new()
			.layout(HorizontalLayout::new(spacing, padding))
			.intrinsic_size(intrinsic_size);
		hbox.arrange(window);

		assert_eq!(hbox.surface.get_size(),window)
	}
}