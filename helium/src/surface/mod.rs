pub mod circle;
pub mod icon;
pub mod image;
pub mod rect;
pub mod text;
use crate::{app::AppState, resources::ResourceManager, widgets::Widget, Bounds, Position, Size};
use crystal::Layout;
use std::fmt::Debug;

/// The surfaces are the items that are actually responsible for drawing the pixels to the
/// screen. It is the final stage in the pipeline, each [`Surface`] holds the data
/// responsible for it's rendering needs, all surfaces, however, hold their [`Position`] and
/// [`Size`] which is calculated during the layout stage. There are currently five surfaces
/// - [`RectSurface`]: drawing rectangular primitives to the screen
/// - [`TextSurface`]: drawing text to the screen
/// - [`CircleSurface`]: drawing circle primitives to the screen
/// - [`ImageSurface`]: drawing images to the screen
/// - [`IconSurface`]: drawing icons to the screen
pub trait Surface: Debug {
    /// Draw the surface onto the screen
    fn draw(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        context: &crate::geometry::RenderContext,
        state: &AppState,
    );

    fn build(&mut self, state: &AppState) {}

    /// Set the [`Position`] of the [`Surface`]
    fn position(&mut self, x: f32, y: f32);

    /// Get the id of the [`Surface`]
    fn id(&self) -> &str;

    /// Get the [`Surface`] position.
    fn get_position(&self) -> Position;

    /// Set the [`Size`] of the [`Surface`].
    fn size(&mut self, width: f32, height: f32);

    /// Set the width of the [`Surface`].
    fn width(&mut self, width: f32);

    /// Set the height of the [`Surface`].
    fn height(&mut self, height: f32);

    /// Get the [`Size`] of the [`Surface`].
    fn get_size(&self) -> Size;

    /// Get the [`Bounds`] of the [`Surface`]
    fn get_bounds(&self) -> Bounds;
}

/// Manages all [`Surface`]'s and their respective resources including
/// - `Buffers`
/// - `Textures`
/// - `Samplers`
/// - `Bind groups`
#[derive(Debug, Default)]
pub struct SurfaceManager {
    resources: ResourceManager,
    surfaces: Vec<Box<dyn Surface>>,
}

impl SurfaceManager {
    /// Create a new [`SurfaceManager`].
    pub fn new(surfaces: Vec<Box<dyn Surface>>) -> Self {
        Self {
            surfaces,
            ..Default::default()
        }
    }

    /// Build the surface manager from the primitives.
    pub fn build(&mut self, widgets: &[Box<dyn Widget>]) {}

    pub fn resize(&mut self, layout: &dyn Layout, state: &AppState) {
        for layout in layout.iter() {
            for surface in &mut self.surfaces {
                if layout.id() == surface.id() {
                    surface.size(layout.size().width, layout.size().height);
                    surface.position(layout.position().x, layout.position().y);
                }
            }
        }
    }

    // FIXME horrible function btw
    /// Rebuild the surfaces
    pub fn rebuild(&mut self, surfaces: Vec<Box<dyn Surface>>) {
        self.surfaces = surfaces;
    }

    pub fn prepare(&mut self, state: &AppState) {
        self.surfaces.iter_mut().for_each(|s| s.build(&state));
    }

    /// Draw the surfaces to the screen
    pub fn draw(&mut self, pass: &mut wgpu::RenderPass, state: &AppState) {
        self.surfaces
            .iter_mut()
            .rev()
            .for_each(|s| s.draw(pass, &state.context, state));
    }
}

#[macro_export]
macro_rules! impl_surface {
    () => {
        fn position(&mut self, x: f32, y: f32) {
            self.position = Position::new(x, y);
        }

        fn get_position(&self) -> Position {
            self.position
        }

        fn id(&self) -> &str {
            &self.id
        }

        fn size(&mut self, width: f32, height: f32) {
            self.size.width = width;
            self.size.height = height;
        }

        fn width(&mut self, width: f32) {
            self.size.width = width
        }

        fn height(&mut self, height: f32) {
            self.size.height = height
        }

        fn get_size(&self) -> Size {
            self.size
        }

        fn get_bounds(&self) -> Bounds {
            let position = self.get_position();
            let size = self.get_size();
            Bounds {
                x: [position.x, size.width],
                y: [position.y, size.height],
            }
        }
    };
}
