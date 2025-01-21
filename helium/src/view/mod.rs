//! [`View`]'s are responsible for drawing [`Widget`]'s to the screen.
//! It is the last stage in the pipeline.
mod circle;
mod icon;
mod image;
mod rect;
mod text;
use crate::{app::AppState, resources::ResourceManager, widgets::Widget, Size};
pub use circle::CircleView;
use crystal::{Layout, Position};
pub use icon::IconView;
pub use image::ImageView;
pub use rect::RectView;
use std::{collections::HashMap, fmt::Debug};
pub use text::TextView;

pub trait View: Debug {
    /// Draws the [`View`] to the screen.
    fn draw(
        &mut self,
        pass: &mut wgpu::RenderPass,
        resources: &ResourceManager,
        context: &crate::geometry::RenderContext,
        state: &AppState,
    );

    /// Initialize the [`View`], this usually involves creating buffers, textures
    /// and bind groups.
    fn init(
        &mut self,
        layout: &dyn Layout,
        resources: &mut ResourceManager,
        state: &AppState,
    ) -> Result<(), crate::Error>;

    fn update(&mut self) {}

    /// Resize the [`View`].
    ///
    /// The behaviour of this is different for each [`View`], for some it's as simple
    /// as writing to a buffer, but for others the textures and images will need to be
    /// resized which can be expensive when done frequently.
    fn resize(
        &mut self,
        layout: &dyn Layout,
        resources: &ResourceManager,
        state: &AppState,
    ) -> Result<(), crate::Error>;

    /// Get the id of the [`View`]
    fn id(&self) -> &str;
}

/// Manages all [`View`]'s and their respective resources.
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
    cache: HashMap<String, (Size, Position)>,
}

impl ViewManager {
    /// Create a new [`SurfaceManager`].
    pub fn new(widget: &impl Widget) -> Self {
        let views: Vec<Box<dyn View>> = widget.iter().map(|w| w.view()).collect();

        Self {
            resources: ResourceManager::new(),
            views,
            cache: HashMap::new(),
        }
    }

    pub fn update(&mut self, widget: &dyn Widget) {
        let views: Vec<Box<dyn View>> = widget.iter().map(|w| w.view()).collect();
    }

    pub fn resize(&mut self, layout: &dyn Layout, state: &AppState) -> Result<(), crate::Error> {
        for view in &mut self.views {
            let (size, position) = self.cache.get(view.id()).unwrap();

            if layout.size() == *size && layout.position() == *position {
                continue;
            }

            let layout = layout
                .get(view.id())
                .ok_or_else(|| crate::Error::NotFound(format!("Layout not found")))?;

            view.resize(layout, &mut self.resources, state)?;
            self.cache
                .insert(layout.id().to_string(), (layout.size(), layout.position()));
        }
        Ok(())
    }

    pub fn build(&mut self, layout: &dyn Layout, state: &AppState) -> Result<(), crate::Error> {
        for view in &mut self.views {
            let layout = layout
                .get(view.id())
                .ok_or_else(|| crate::Error::NotFound(format!("Layout not found")))?;
            view.init(layout, &mut self.resources, state)?;
            self.cache
                .insert(layout.id().to_string(), (layout.size(), layout.position()));
        }

        Ok(())
    }

    /// Draw the surfaces to the screen
    pub fn draw(&mut self, pass: &mut wgpu::RenderPass, state: &AppState) {
        self.views
            .iter_mut()
            .for_each(|s| s.draw(pass, &self.resources, &state.context, state));
    }
}
