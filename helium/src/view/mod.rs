mod circle;
mod icon;
mod image;
mod rect;
mod text;
pub use circle::CircleView;
pub use image::ImageView;
pub use rect::RectView;
pub use text::TextView;
pub use icon::IconView;
use crate::{
    app::AppState, resources::ResourceManager, widgets::Widget, Size
};
use crystal::Layout;
use std::{collections::HashMap, fmt::Debug};

// TODO update docs
/// The surfaces are the items that are actually responsible for drawing the pixels to the
/// screen. It is the final stage in the pipeline, each [`View`] holds the data
/// responsible for it's rendering needs, all surfaces, however, hold their [`Position`] and
/// [`Size`] which is calculated during the layout stage. There are currently five surfaces
/// - [`RectSurface`]: drawing rectangular primitives to the screen
/// - [`TextSurface`]: drawing text to the screen
/// - [`CircleSurface`]: drawing circle primitives to the screen
/// - [`ImageSurface`]: drawing images to the screen
/// - [`IconSurface`]: drawing icons to the screen
pub trait View: Debug {
    /// Draw the surface onto the screen
    fn draw(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        resources: &ResourceManager,
        context: &crate::geometry::RenderContext,
        state: &AppState,
    );
	
	/// Initialize the [`View`], this usually involves creating buffers, textures
	/// and bind groups.
	fn init(
		&mut self,
		layout:&dyn Layout,
		resources:&mut ResourceManager,
		state: &AppState
	) -> Result<(),crate::Error>;

    /// Get the id of the [`View`]
    fn id(&self) -> &str;
}

// enum PipelineState {
//     Compute(ComputePipeline),
//     Render(RenderPipeline),
// }

// struct WgpuShader {
//     pipeline: PipelineState,
//     bind_group_layout: BindGroupLayout,
// }


/// Manages all [`View`]'s and their respective resources including
/// - `Buffers`
/// - `Textures`
/// - `Samplers`
/// - `Bind groups`
#[derive(Debug)]
pub struct ViewManager {
    resources: ResourceManager,
    views: Vec<Box<dyn View>>,
    /// A cache of all the sizes of the surfaces.  
    ///
    /// Resizing some surfaces is expensive, particularly the
    /// [`ImageSurface`], because an entirely new `Texture` will
    /// have to be created and written to. So only [`Surfaces`]'s
    /// whose size has actually changed will be resized.
    size_cache: HashMap<String, Size>,
}

impl ViewManager {
    /// Create a new [`SurfaceManager`].
    pub fn new(widget: &impl Widget) -> Self {
        //let primitives: Vec<Primitive> = widget.iter().map(|w| w.primitive()).collect();

        Self {
            resources: ResourceManager::new(),
            views: vec![],
            size_cache: HashMap::new(),
        }
    }

    /// Build the [`View`]'s from the [`Primitive`]'s.
    pub fn build(&mut self,layout: &dyn Layout, state: &AppState) {
        self.surfaces = self
            .primitives
            .iter()
            .map(|primitive| primitive.build(&mut self.resources, &state))
            .collect();

		self.resize(layout, state);

        self.surfaces
            .iter_mut()
            .for_each(|s| s.build(state, &self.resources));
    }

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

    /// Draw the surfaces to the screen
    pub fn draw(&mut self, pass: &mut wgpu::RenderPass, state: &AppState) {
        self.surfaces
            .iter_mut()
            .for_each(|s| s.draw(pass, &self.resources, &state.context, state));
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
