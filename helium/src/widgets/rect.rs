use std::fmt::Debug;
use super::{Widget, WidgetBody};
use crate::app::events::{Event};
use crate::color::Color;
use crate::{impl_events, impl_interative};
use crate::layout::{IntrinsicSize, Layout, WidgetSize};
use crate::surface::rect::RectSurface;
use crate::utils::{Position, Size};

#[derive(Debug)]
/// A simple rectangle
pub struct Rect {
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub hover_func: Event<Self>,
	pub events: Vec<Event<Self>>
}

impl Rect {
    pub fn new(width: f32, height: f32, color: Color) -> Self {
        Self {
            width,
            height,
            color,
            hover_func:Event::OnHover(Box::new(|_|{})),
			events: Vec::new()
        }
    }

	impl_events!(Rect);
}

impl Widget for Rect {
    fn build(&self) -> WidgetBody {
        let layout = Layout::Block { padding: 0 };
        let surface = Box::new(RectSurface {
            size: Size::new(self.width as f32, self.height as f32),
            color: self.color.clone(),
            ..Default::default()
        });


        WidgetBody {
            surface,
            layout,
            children: vec![],
            intrinsic_size: IntrinsicSize {
                width: WidgetSize::Fixed(self.width),
                height: WidgetSize::Fixed(self.height),
            },
            ..Default::default()
        }
    }

	impl_interative!();
}

