use super::Widget;
use crate::{
    events::{EventContext, EventFn}, view::RectView, Color
};
use crystal::{BoxSizing, EmptyLayout, IntrinsicSize, Layout};
use helium_core::color::WHITE;
use nanoid::nanoid;

/// A simple rectangle
#[derive(Debug,Clone, PartialEq,PartialOrd)]
pub struct Rect {
    id: String,
    intrinsic_size: crystal::IntrinsicSize,
    color: Color,
    corner_radius: u32,
}

impl Rect {
    pub fn new(width: f32, height: f32) -> Self {
        let intrinsic_size = IntrinsicSize {
            width: BoxSizing::Fixed(width),
            height: BoxSizing::Fixed(height),
        };

        Self {
            id: nanoid!(),
            color:WHITE,
            intrinsic_size,
            corner_radius: 0,
        }
    }

	pub fn color(mut self, color:Color) -> Self{
		self.color = color;
		self
	}

	/// This event fires when the mouse cursor is over a [`Widget`]
	/// 
	/// # Example
	/// ```
	/// use helium::widgets::Rect;
	/// 
	/// Rect::new(150.0,150.0)
	/// 	.on_hover(||println!("Hello world"));
	/// ```
    pub fn on_hover(self,cx:&mut EventContext, f: impl FnMut() + 'static) -> Self {
		let event = EventFn::OnHover(Box::new(f));
		cx.add(&self.id,event);
        self
    }

	/// This event fires when the mouse clicks on a [`Widget`]
	/// 
	/// # Example
	/// ```
	/// use helium::widgets::Rect;
	/// 
	/// Rect::new(150.0,150.0)
	/// 	.on_click(||println!("Hello world"));
	/// ```
    pub fn on_click(self,cx:&mut EventContext, f: impl FnMut() + 'static) -> Self {
        let event = EventFn::OnClick(Box::new(f));
		cx.add(&self.id,event);
        self
    }

    /// Set the corner radius
    pub fn corner_radius(mut self, corner_radius: u32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    // TODO replace with impl_widget!()
    pub fn flex_width(mut self, factor: u8) -> Self {
        self.intrinsic_size.width = BoxSizing::Flex(factor);
        self
    }

    pub fn flex_height(mut self, factor: u8) -> Self {
        self.intrinsic_size.height = BoxSizing::Flex(factor);
        self
    }
}

impl Widget for Rect {
    fn id(&self) -> &str {
        &self.id
    }

    fn layout(&self) -> Box<dyn Layout> {
        let mut layout = EmptyLayout::new();
        layout.intrinsic_size = self.intrinsic_size;
        layout.id = self.id.clone();

        Box::new(layout)
    }

    fn view(&self) -> Box<dyn crate::view::View> {
        Box::new(
            RectView::new(&self.id)
                .color(self.color)
                .corner_radius(self.corner_radius),
        )
    }
}

