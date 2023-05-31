use crate::state::action::{ActionResult, KeyBinding};
use crate::state::view::{R, View};

pub trait Layer {
	fn handle_input(&mut self, key_binding: KeyBinding) -> ActionResult;
	fn render(&mut self, view: &mut View) -> R;
}
