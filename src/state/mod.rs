use std::path::Path;

use crate::component::filesystem::FsLayer;
use crate::input::keymap::KeyBinding;
use crate::state::action::ActionResult;
use crate::state::layer::Layer;
use crate::state::view::Frame;

pub use self::environment::Environment;

mod environment;
pub mod action;
pub mod layer;
pub mod view;

pub struct State {
	layers: Vec<Box<dyn Layer>>,
	environment: Environment,
}

impl State {
	pub fn with_root_path(root_path: &Path, environment: Environment) -> Self {
		Self {
			layers: vec![Box::new(FsLayer::with_root_path(root_path))],
			environment
		}
	}
	
	pub fn handle_input(&mut self, key_binding: KeyBinding) -> ActionResult {
		self.layers.last_mut().map(|layer| layer.handle_input(&self.environment, key_binding)).unwrap_or(ActionResult::Nothing)
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
