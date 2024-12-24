use helium_core::{position::Position, size::Size};
use crate::{BoxContraints, BoxSizing, IntrinsicSize, Layout, LayoutIter};

/// This is a layout with no children
#[derive(Debug,Default,Clone)]
pub struct EmptyLayout{ // TODO add padding
	pub id:String,
	size:Size,
	position:Position,
	pub intrinsic_size:IntrinsicSize,
	constraints:BoxContraints,
}

impl EmptyLayout {
	pub fn new() -> Self{
		Self::default()
	}
}


impl Layout for EmptyLayout {
	fn size(&self) -> Size {
		self.size
	}

	fn id(&self) -> &str {
		&self.id
	}
	
	fn set_position(&mut self,position:Position) {
		self.position = position;
	}

	fn set_x(&mut self,x:f32) {
		self.position.x = x;
	}

	fn set_y(&mut self,y:f32) {
		self.position.y = y;
	}

	fn position(&self) -> Position {
		self.position
	}

	fn children(&self) -> &[Box<dyn Layout>] {
		&[]
	}

	fn constraints(&self) -> BoxContraints {
		self.constraints
	}

	fn intrinsic_size(&self) -> IntrinsicSize {
		self.intrinsic_size
	}

	fn set_max_height(&mut self,height:f32) {
		self.constraints.max_height = height;
	}
	
	fn set_max_width(&mut self,width:f32) {
		self.constraints.max_width = width;
	}
	
	fn set_min_height(&mut self,height:f32) {
		self.constraints.min_height = height;
	}
	
	fn set_min_width(&mut self,width:f32) {
		self.constraints.min_width = width;
	}

	fn iter(&self) -> crate::LayoutIter {
		LayoutIter{
			stack:vec![Box::new(self)]
		}
	}

	fn solve_min_constraints(&mut self) -> (f32,f32){
		// The sum of the size of all the children with fixed sizes

		match self.intrinsic_size.width {
			BoxSizing::Fixed(width) => {
				self.constraints.min_width = width;	
			},
			BoxSizing::Flex(_) => {
			},
			BoxSizing::Shrink => {},
		}
		
		match self.intrinsic_size.height {
			BoxSizing::Fixed(height) => {
				self.constraints.min_height = height;	
			},
			BoxSizing::Flex(_) => {

			},
			BoxSizing::Shrink => {},
		}

		(self.constraints.min_width,self.constraints.min_height)
	}

	fn solve_max_contraints(&mut self,space:Size) {}

	fn update_size(&mut self){
		match self.intrinsic_size.width {
			BoxSizing::Flex(_) => {
				self.size.width = self.constraints.max_width;
			},
			BoxSizing::Shrink => {
				self.size.width = self.constraints.min_width;
			},
			BoxSizing::Fixed(width) => {
				// TODO maybe set the min constrains?
				self.size.width = width;
			}
		}

		match self.intrinsic_size.height {
			BoxSizing::Flex(_) => {
				self.size.height = self.constraints.max_height;
			},
			BoxSizing::Shrink => {
				self.size.height = self.constraints.min_height;
			},
			BoxSizing::Fixed(height) => {
				// TODO maybe set the min constrains?
				self.size.height = height;
			}
		}
	}

	fn position_children(&mut self){}
}

#[cfg(test)]
mod test{
	use crate::LayoutSolver;
	use super::*;

	#[test]
	fn test_flex_sizing(){
		let window = Size::new(800.0, 800.0);
		let mut root = EmptyLayout::new();

		root.intrinsic_size.width = BoxSizing::Flex(2);
		root.intrinsic_size.height = BoxSizing::Flex(2);
		
		LayoutSolver::solve(&mut root,window);
		
		assert_eq!(root.size(),window);
	}

	#[test]
	fn test_fixed_sizing(){
		let window = Size::new(800.0, 800.0);
		let mut root = EmptyLayout::new();
		
		root.intrinsic_size.width = BoxSizing::Fixed(200.0);
		root.intrinsic_size.height = BoxSizing::Fixed(125.0);

		LayoutSolver::solve(&mut root,window);

		assert_eq!(root.size(),Size::new(200.0, 125.0));
	}

	#[test]
	fn test_shrink_sizing(){
		let window = Size::new(800.0, 800.0);
		let mut root = EmptyLayout::new();

		LayoutSolver::solve(&mut root, window);

		assert_eq!(root.size(),Size::default());
	}
}