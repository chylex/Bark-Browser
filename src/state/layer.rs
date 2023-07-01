use crate::input::keymap::KeyBinding;
use crate::state::action::ActionResult;
use crate::state::Environment;
use crate::state::view::F;

pub trait Layer {
	fn handle_input(&mut self, environment: &Environment, key_binding: KeyBinding) -> ActionResult;
	fn render(&mut self, frame: &mut F);
}
