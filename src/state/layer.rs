use crate::input::keymap::KeyBinding;
use crate::state::action::ActionResult;
use crate::state::Environment;
use crate::state::event::EventResult;
use crate::state::view::Frame;

pub trait Layer {
	fn handle_input(&mut self, environment: &Environment, key_binding: KeyBinding) -> ActionResult;
	fn handle_events(&mut self, environment: &Environment) -> EventResult;
	fn render(&mut self, frame: &mut Frame);
}
