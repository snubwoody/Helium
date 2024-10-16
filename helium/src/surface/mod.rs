pub mod rect;
pub mod text;
pub mod image;
use std::fmt::Debug;
use crate::utils::{Bounds, Position, Size};

/// A primitive object that is drawn to the screen
pub trait Surface:Debug {
	/// Draw the surface onto the screen
	fn draw(
		&self,
		render_pass:&mut wgpu::RenderPass,
		context: &crate::app::RenderContext,
		device:&wgpu::Device
	);

	/// Set the [`Position`] of the [`Surface`]
	fn position(&mut self, x:f32,y:f32);	
	
	/// Get the [`Surface`] position.
	fn get_position(&self) -> Position;

	/// Set the [`Size`] of the [`Surface`].
	fn size(&mut self,width:f32,height:f32);

	/// Set the width of the [`Surface`].
	fn width(&mut self, width:f32);
	
	/// Set the height of the [`Surface`].
	fn height(&mut self, height:f32);

	/// Get the [`Size`] of the [`Surface`].
	fn get_size(&self) -> Size;

	/// Get the [`Bounds`] of the [`Surface`]
	fn get_bounds(&self) -> Bounds;
}

