//! The surfaces are the items that are actually responsible for drawing the pixels to the 
//! screen. It is the final stage in the pipeline, each [`Surface`] surface holds the data 
//! responsible for it's rendering needs, all surfaces, however, hold their [`Position`] and
//! [`Size`] which is calculated during the layout stage. There are currently five surfaces
//! - [`RectSurface`]: drawing rectangular primitives to the screen
//! - [`TextSurface`]: drawing text to the screen
//! - [`CircleSurface`]: drawing circle primitives to the screen
//! - [`ImageSurface`]: drawing images to the screen
//! - [`IconSurface`]: drawing icons to the screen
pub mod rect;
pub mod text;
pub mod circle;
pub mod image;
pub mod icon;
use std::fmt::Debug;
use crate::{app::AppState, Bounds, Position, Size};

// TODO maybe change to primitive?
/// The surfaces are the items that are actually responsible for drawing the pixels to the 
/// screen. It is the final stage in the pipeline, each [`Surface`] surface holds the data 
/// responsible for it's rendering needs.
pub trait Surface:Debug {
	/// Draw the surface onto the screen
	fn draw(
		&mut self,
		render_pass:&mut wgpu::RenderPass,
		context: &crate::geometry::RenderContext,
		state:&AppState
	);

	/// Set the [`Position`] of the [`Surface`]
	fn position(&mut self, x:f32,y:f32);	
	
	/// Get the [`Surface`] [`Position`].
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

#[macro_export]
macro_rules! impl_surface {
	() => {
		fn position(&mut self, x:f32,y:f32){
			self.position = Position::new(x, y);
		} 
		
		fn get_position(&self) -> Position {
			self.position
		} 
	
		fn size(&mut self,width:f32,height:f32){
			self.size.width = width;
			self.size.height = height;
		} 
	
		fn width(&mut self, width:f32) {
			self.size.width = width
		}
	
		fn height(&mut self, height:f32) {
			self.size.height = height
		}
	
		fn get_size(&self) -> Size {
			self.size
		}
	
		fn get_bounds(&self) -> Bounds{
			let position = self.get_position();
			let size = self.get_size();
			Bounds{
				x:[position.x,size.width],
				y:[position.y,size.height],
			}
		}
	};
}

