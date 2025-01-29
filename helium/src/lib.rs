//! A gui library built using `wgpu`. It uses an entirely custom renderer for drawing
//! the ui and uses the `crystal` crate for layout.
pub mod colors;
pub mod error;
pub mod widgets;
pub use crystal;
use crystal::LayoutSolver;
pub use error::Error;
pub use helium_core::color::*; 
pub use helium_core::position::{Bounds, Position};
pub use helium_core::size::Size;
pub use helium_macros::hex; // TODO move to colors mod
pub use nanoid::nanoid;
use std::time::{Duration, Instant};
use helium_renderer::{Renderer, Text};
use winit::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use widgets::Widget;
// TODO re-export the winit event modules 

/// [`App`]'s contain the whole program and are the point of entry for helium
/// they are responsible for the overall management of rendering, resources,
/// [`Widget`]'s etc.
pub struct App{
    event_loop: EventLoop<()>,
    window: Window,
    pages: Vec<Page>,
    index: usize,
}

impl App {
    pub fn new() -> Self {
        // FIXME handle the errors
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let window = WindowBuilder::new()
            .with_visible(false)
            .build(&event_loop)
            .unwrap();
		

        Self {
            event_loop,
            window,
            pages: vec![],
            index: 0,
        }
    }

    pub fn add_page(mut self, page: Page) -> Self {
        self.pages.push(page);
        self
    }

    // FIXME app panics if there are no views
    pub fn run(mut self) -> Result<(), crate::Error> {
        self.window.set_visible(true);

		let mut renderer = async_std::task::block_on(Renderer::new(&self.window));
		log::info!("Running app");

		// Not quite sure how accurate this is
		let mut previous_duration = Duration::new(0, 0);
		let mut size = Size::from(self.window.inner_size());

        self.event_loop
            .run(|event, window_target|{ 
				let instant = Instant::now();
				match event {
				winit::event::Event::WindowEvent { event, .. } => match event {
					WindowEvent::CloseRequested => window_target.exit(),
					WindowEvent::RedrawRequested => {
						self.pages[0].draw(&mut renderer,size);
						renderer.draw([
							Text::new(format!("{:?}",previous_duration).as_str())
						]);
						renderer.render();
					}
					WindowEvent::Resized(window_size) => {
						size = window_size.into();
						self.pages[self.index].resize(Size::from(window_size));
						renderer.resize(window_size.into());
						// I think resizing already causes a redraw request but i'm not sure
						self.window.request_redraw(); 
					},
					event => {
						self.pages[self.index].dispatch_event(&event);
						self.window.request_redraw();
					},
				},
				_ => {}
            	}
				previous_duration = instant.elapsed();
		})?;

        Ok(())
    }
}

pub struct Page {
    layout: Box<dyn crystal::Layout>,
    widget: Box<dyn Widget>,
}

impl Page {
    pub fn new(widget: impl Widget + 'static) -> Self {
        Self {
            layout: widget.layout(),
            widget: Box::new(widget),
        }
    }

    fn resize(&mut self, size:Size) {
        LayoutSolver::solve(&mut *self.layout,size);
    }

	fn dispatch_event(&mut self,event: &winit::event::WindowEvent){
		self.widget.dispatch_event(&*self.layout,event);
	}

	fn draw(&self, renderer:&mut Renderer,size:Size){
		let mut layout = self.widget.layout();
		LayoutSolver::solve(&mut *layout, size);

		self.widget.iter().for_each(|w|{
			if let Some(layout) = layout.get(w.id()){
				// TODO add an error or similar here; every widget should have a layout
				w.draw(layout,renderer); 
			}else {
				log::warn!("Widget has missing Layout")
			}
		});
	}
}

