use crate::{
    impl_style, impl_widget,
    surface::rect::RectSurface,
    widgets::{Widget, WidgetBody},
    Color,
};
use crystal::{AxisAlignment, Layout, VerticalLayout};
use helium_core::color::TRANSPARENT;

pub struct VStack {
	id:String,
    children: Vec<Box<dyn Widget>>,
    color: Color,
    layout: VerticalLayout,
}

impl VStack {
    pub fn new() -> Self {
        VStack {
			id:nanoid::nanoid!(),
            color: TRANSPARENT,
            children: vec![],
            layout: VerticalLayout::new(),
        }
    }

	pub fn add_child(mut self,widget:impl Widget + 'static) -> Self{
		self.children.push(Box::new(widget));
		self
	}

    pub fn padding(mut self, padding: u32) -> Self {
        self.layout.padding = padding;
        self
    }

    pub fn spacing(mut self, spacing: u32) -> Self {
        self.layout.spacing = spacing;
        self
    }

    pub fn main_axis_alignment(mut self, alignment: AxisAlignment) -> Self {
        self.layout.main_axis_alignment = alignment;
        self
    }

    pub fn cross_axis_alignment(mut self, alignment: AxisAlignment) -> Self {
        self.layout.cross_axis_alignment = alignment;
        self
    }

    impl_widget!();
    impl_style!();
}

impl Widget for VStack {
    fn build(&self) -> (WidgetBody, Box<dyn Layout>) {
        let mut surface = RectSurface::new(&self.id);
        surface.color(self.color);

        let (children_body, children_layout): (Vec<Box<WidgetBody>>, Vec<Box<dyn Layout>>) = self
            .children
            .iter()
            .map(|widget| {
                let (body, layout) = widget.build();
                (Box::new(body), layout)
            })
            .collect();

        let body = WidgetBody {
            id: self.id.clone(),
            children: children_body,
            surface: Box::new(surface),
            ..Default::default()
        };

        let VerticalLayout {
            spacing,
            padding,
            intrinsic_size,
            main_axis_alignment,
            cross_axis_alignment,
            constraints,
            ..
        } = self.layout;

        let layout = VerticalLayout {
            id: self.id.clone(),
            spacing,
            padding,
            intrinsic_size,
            constraints,
            main_axis_alignment,
            cross_axis_alignment,
            children: children_layout,
            ..Default::default()
        };

        (body, Box::new(layout))
    }

	fn surface(&self) -> Vec<Box<dyn crate::surface::Surface>> {
		let mut surfaces = self
			.children
			.iter()
            .flat_map(|widget| {
				widget.surface()
            })
            .collect::<Vec<_>>();

		let mut surface = RectSurface::new(&self.id);
		surface.color(self.color.clone());
	
		surfaces.push(Box::new(surface));

		surfaces
	}

    fn update(&mut self) {
        self.children.iter_mut().for_each(|child| child.update());
    }
}

/// Creates an [`VStack`].  
///
/// `vstack!` allows [`VStack`]'s to be declared in a more declarative manner.  
///
/// ```
/// use helium::{vstack,widgets::{Button,Text}};
///
/// vstack!{
/// 	Button::new("Click me"),
/// 	Text::new("Hello world")
/// };
/// ```
#[macro_export]
macro_rules! vstack {
	($($child:expr), + $(,)?) => {
		{
			$crate::widgets::VStack::new()
			$(.add_child($child))*
		}

	};
}
