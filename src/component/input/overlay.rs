use crossterm::event::{KeyCode, KeyModifiers};

use crate::component::input::InputField;
use crate::input::keymap::KeyBinding;
use crate::state::action::ActionResult;
use crate::state::Environment;
use crate::state::layer::Layer;
use crate::state::view::F;

pub struct InputFieldOverlayLayer {
	field: InputField,
	confirm_action: Box<dyn Fn(String) -> ActionResult>,
}

impl InputFieldOverlayLayer {
	pub fn new(confirm_action: Box<dyn Fn(String) -> ActionResult>) -> Self {
		Self {
			field: InputField::new(),
			confirm_action,
		}
	}
}

impl Layer for InputFieldOverlayLayer {
	#[allow(clippy::wildcard_enum_match_arm)]
	fn handle_input(&mut self, _environment: &Environment, key_binding: KeyBinding) -> ActionResult {
		match (key_binding.code(), key_binding.modifiers()) {
			(KeyCode::Esc, KeyModifiers::NONE) |
			(KeyCode::Char('c'), KeyModifiers::CONTROL) => {
				ActionResult::PopLayer
			}
			
			(KeyCode::Enter, KeyModifiers::NONE) => {
				(self.confirm_action)(self.field.text().to_owned())
			}
			
			_ => {
				if self.field.handle_input(key_binding) {
					ActionResult::Draw
				} else {
					ActionResult::Nothing
				}
			}
		}
	}
	
	fn render(&mut self, frame: &mut F) {
		let size = frame.size();
		self.field.render(frame, size.x, size.bottom().saturating_sub(1), size.width);
	}
}
