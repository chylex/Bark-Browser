use crate::input::keymap::KeyBinding;
use crate::state::action::ActionResult;
use crate::state::view::F;

pub trait Layer {
	fn handle_input(&mut self, key_binding: KeyBinding) -> ActionResult;
	fn render(&mut self, frame: &mut F);
}
