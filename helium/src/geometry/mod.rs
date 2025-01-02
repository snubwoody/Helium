pub mod vertex;
pub mod uniform;
mod pipeline;

pub use pipeline::text::TextPipeline;
pub use pipeline::rect::RectPipeline;
pub use pipeline::circle::CirclePipeline;
pub use pipeline::RenderContext;