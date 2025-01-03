use std::fmt::Debug;
use winit::event::WindowEvent;
use crate::{widgets::WidgetBody, Position};

type EventFunction = Box<dyn FnMut()>; 
pub enum Event {
	OnClick(EventFunction),
	OnHover(EventFunction),
}

#[derive(Debug)]
pub struct UserEvent {
	function:Event,
	id:String
}

impl UserEvent {
	pub fn new(id:String,f:Event) -> Self{
		Self { 
			function:f,
			id,
		}
	}
}

impl Debug for Event {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self {
			Self::OnClick(_) => {
				f.debug_tuple("OnClick()").finish()
			},
			Self::OnHover(_) => {
				f.debug_tuple("OnHover()").finish()
			},
		}
	}
}


#[derive(Debug)]
pub struct EventQueue{
	cursor_pos:Position,
	_loop:Vec<UserEvent>
}

impl EventQueue {
	pub fn new() -> Self{
		Self { 
			cursor_pos:Position::default(),
			_loop:vec![]
		}
	}

	pub fn push(&mut self,event:UserEvent){
		self._loop.push(event);
	}

	/// Check if the cursor is over the [`Widget`]
	pub fn check_click(&mut self,root_body:&WidgetBody){
		// FIXME it's triggering slightly outside
		let bounds = root_body.surface.get_bounds();

		if !bounds.within(&self.cursor_pos){
			return;
		}

		for e in &mut self._loop{
			if e.id != root_body.id{continue}
			match &mut e.function {
				Event::OnClick(func) => func(),
				_ => {}
			}
		}
	}

	pub fn handle_events(
		&mut self,
		event:&winit::event::WindowEvent,
		root_body:&WidgetBody
	) {
		match event {
			WindowEvent::MouseInput { state, button,.. } => {
				match button {
					winit::event::MouseButton::Left => {
						match state {
							winit::event::ElementState::Pressed => {
								self.check_click(root_body);
							},
							winit::event::ElementState::Released => {

							}
						}
					},
					_ => {}
				}
			},
			WindowEvent::CursorMoved { position,.. } => {
				// Update the cursor position every time it moves
				self.cursor_pos = Position::from(*position);
			}
			_ => {}
		}

		root_body.children.iter().for_each(|child|self.handle_events(event, &child));
	}
}

#[macro_export]
macro_rules! impl_events {
	() => {
		pub fn on_click(self,event_loop:&mut $crate::app::events::EventQueue,f:impl FnMut() + 'static) -> Self{
			event_loop.push($crate::app::events::UserEvent::new(self.id.clone(), Event::OnClick(Box::new(f))));
			self
		}
	};
}
