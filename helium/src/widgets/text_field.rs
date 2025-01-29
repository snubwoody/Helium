use super::{Text, Widget};
use crystal::{BlockLayout, BoxSizing, EmptyLayout, IntrinsicSize, Layout};
use helium_core::color::Color;
use helium_renderer::Rect;
use wgpu::hal::auxil::db;

/// Contains editable text
pub struct TextField {
    id: String,
    text: Option<Text>,
	focused:bool,
	/// The background color when this widget is focused.
	pub focus_background_color:Color,
    pub background_color: Color,
	pub corner_radius:u32
}

impl TextField {
    pub fn new() -> Self {
        Self {
            id: nanoid::nanoid!(),
            text: None,
			focused:false,
            focus_background_color: Color::default(),
            background_color: Color::default(),
			corner_radius:12
        }
    }

    pub fn on_click(mut self, f: impl FnMut() + 'static) -> Self {
        self
    }

	/// Set the background color of the [`TextField`] when it is focused.
	pub fn focus_background_color(mut self,focus_background_color:Color) -> Self{
		self.focus_background_color = focus_background_color;
		self
	}
	
	/// Set the background color of the [`TextField`].
	pub fn background_color(mut self,background_color:Color) -> Self{
		self.background_color = background_color;
		self
	}

    fn on_input(&mut self, f: impl FnMut(&str) + 'static) {

	}
}

impl Widget for TextField {
    fn id(&self) -> &str {
        &self.id
    }

	fn unfocus(&mut self) {
		self.focused = false;
	}

	fn click(&mut self) {
		self.focused = true;
	}

	fn process_key(&mut self,key:&winit::keyboard::Key) {
		if !self.focused{
			return;
		}

		match key {
			winit::keyboard::Key::Character(character) => {
				match &self.text {
					Some(text) => {
						let mut content = String::from(&text.text);
						content.push_str(&character);
						self.text = Some(Text::new(&content))
					},
					None => {
						self.text = Some(Text::new(&character))
					}
				}
			},
			winit::keyboard::Key::Named(named_key) => {
				match named_key {
					winit::keyboard::NamedKey::Backspace => {
						if let Some(text) = &mut self.text {
							if text.text.len() == 1{
								self.text = None;
								return;
							}
							text.text.pop(); // FIXME panics
						};
					},
					winit::keyboard::NamedKey::Space => {
						if let Some(text) = &mut self.text {
							text.text.push_str(" ");
						};
					}
					_ => {}
				}
			},
			_ => {}
		}
	}

    fn layout(&self) -> Box<dyn crystal::Layout> {
		let child = match &self.text {
			Some(text) => text.layout(),
			None => {
				let mut child_layout = EmptyLayout::new();
				Box::new(child_layout)
			}
		};

        let mut layout = BlockLayout::new(child);
		layout.id = self.id.clone();
		layout.intrinsic_size.width = BoxSizing::Fixed(200.0);
		layout.padding = 12;
        Box::new(layout)
    }

	fn children(&self) -> Vec<&dyn Widget> {
		match &self.text {
			Some(text) => vec![text],
			None => vec![]
		}
	}

	fn draw(&self,layout:&dyn crystal::Layout,renderer:&mut helium_renderer::Renderer) {
		let background_color = match self.focused{
			true => self.background_color,
			false => self.focus_background_color,
		};
		
		renderer.draw([
			Rect::from(layout)
				.color(background_color)
				.corner_radius(self.corner_radius as f32)
		]);
		// self.text.draw(&*layout.children()[0], renderer);
	}
}


#[cfg(test)]
mod tests{
	use super::*;
	use winit::keyboard::{SmolStr,Key,NamedKey};

	#[test]
	fn text_updates_on_key_input(){
		
		let mut text_field = TextField::new();

		let keys = [
			Key::Character(SmolStr::new("H")),
			Key::Character(SmolStr::new("i")),
			Key::Character(SmolStr::new(" ")),
			Key::Character(SmolStr::new("m")),
			Key::Character(SmolStr::new("o")),
			Key::Character(SmolStr::new("m")),
			Key::Character(SmolStr::new("!"))
		];


		for key in keys{
			text_field.process_key(&key);
		}

		assert_eq!(text_field.text.unwrap().text,"Hi mom!")
	}

	#[test]
	fn backspace_deletes_text(){
		
		let mut text_field = TextField::new();
		text_field.text = Some(Text::new("Hello"));

		let keys = [
			Key::Named(NamedKey::Backspace),
			Key::Named(NamedKey::Backspace)
		];


		for key in keys{
			text_field.process_key(&key);
		}

		assert_eq!(text_field.text.unwrap().text,"Hel")
	}

	#[test]
	fn space_key_adds_space(){
		todo!()
	}
}