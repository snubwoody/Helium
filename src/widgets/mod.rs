pub mod rect;
pub mod stack;
pub mod container;
use glium::{
	glutin::surface::WindowSurface, Display, Frame, Program,
};
use winit::window::Window;

use crate::surface::Surface;

/// Widget trait that all widgets must inherit from
pub trait Widget {
	fn render(
		&mut self,
		display:&Display<WindowSurface>,
		frame:&mut Frame,
		window:&Window,
		program:&Program,
	);
	/// Set the position of the [`Widget`]  
	/// Note that positions start from the upper left 
	/// corner
	// TODO change to position
	fn position(&mut self,x:i32,y:i32);
	//fn get_surface(&self) -> Surface;	
	//TODO change to get_size then add function size that sets the size 
	// to be more idiomatic
	///Returns the size
	fn get_size(&mut self) -> [i32;2];
}