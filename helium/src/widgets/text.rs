use super::Widget;
use crystal::{BoxSizing, EmptyLayout, Layout};
use helium_core::color::Color;
use helium_renderer::IntoPrimitive;

// TODO add editable() method?
// TODO TextStyle struct
/// A [`Widget`] for displaying text onto the screen.
///
/// # Example
/// ```
/// use helium::widgets::Text;
///
/// Text::new("Hello world");
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct Text {
    id: String,
    pub text: String,
    pub font_size: u8,
    pub color: Color,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            id: nanoid::nanoid!(),
            font_size: 16,
            text: Default::default(),
            color: Color::Hex("#000000"),
        }
    }
}

impl Text {
    pub fn new(text: &str) -> Self {
        Self {
            id: nanoid::nanoid!(),
            text: text.into(),
            font_size: 16,
            color: Color::Hex("#000000"),
        }
    }

    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the font size
    pub fn font_size(mut self, size: u8) -> Self {
        self.font_size = size;
        self
    }

    fn primitive(&self) -> helium_renderer::Text {
        helium_renderer::Text::new(&self.text)
            .font_size(self.font_size)
            .color(self.color)
    }
}

impl Widget for Text {
    fn id(&self) -> &str {
        &self.id
    }

    fn layout(&self, renderer: &mut helium_renderer::Renderer) -> Box<dyn Layout> {
        let text = self.primitive();
        let size = renderer.text_size(&text);

        let mut layout = EmptyLayout::new();
        layout.intrinsic_size.width = BoxSizing::Fixed(size.width);
        layout.intrinsic_size.height = BoxSizing::Fixed(size.height);
        layout.id = self.id.clone();

        Box::new(layout)
    }

    fn draw(&self, layout: &dyn Layout, renderer: &mut helium_renderer::Renderer) {
        let position = layout.position();
        renderer.draw([self.primitive().position(position.x, position.y)]);
    }
}
