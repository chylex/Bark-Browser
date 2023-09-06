use crate::component::filesystem::FsLayer;
use crate::input::keymap::KeyBinding;
use crate::state::action::ActionResult;
use crate::state::event::EventResult;
use crate::state::init::StateInitializer;
use crate::state::layer::Layer;
use crate::state::view::Frame;

pub use self::environment::Environment;

mod environment;
pub mod action;
pub mod event;
pub mod init;
pub mod layer;
pub mod view;

pub struct State {
	layers: Vec<Box<dyn Layer>>,
	environment: Environment,
}

impl State {
	pub fn new(initializer: &StateInitializer, environment: Environment) -> Self {
		Self {
			layers: vec![Box::new(FsLayer::new(initializer.filesystem_start_path, initializer.filesystem_action_map))],
			environment
		}
	}
	
	pub fn handle_events(&mut self) -> EventResult {
		self.layers.iter_mut().fold(EventResult::Nothing, |result, layer| result.merge(layer.handle_events(&self.environment)))
	}
	
	pub fn handle_input(&mut self, key_binding: KeyBinding) -> ActionResult {
		self.layers.last_mut().map_or(ActionResult::Nothing, |layer| layer.handle_input(&self.environment, key_binding))
	}
	
	pub fn handle_resize(&mut self, width: u16, height: u16) {
		self.environment.terminal_width = width;
		self.environment.terminal_height = height;
	}
	
	pub fn render(&mut self, frame: &mut Frame) {
		for layer in &mut self.layers {
			frame.hide_cursor();
			layer.render(frame);
		}
	}
	
	pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
		self.layers.push(layer);
	}
	
	pub fn pop_layer(&mut self) -> bool {
		self.layers.pop();
		self.layers.is_empty()
	}
}
