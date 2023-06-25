use std::path::Path;

use crate::component::filesystem::FsLayer;
use crate::input::keymap::KeyBinding;
use crate::state::action::ActionResult;
use crate::state::layer::Layer;
use crate::state::view::F;

pub mod action;
pub mod layer;
pub mod view;

pub struct State {
	layers: Vec<Box<dyn Layer>>,
}

impl State {
	pub fn with_root_path(root_path: &Path) -> Self {
		Self {
			layers: vec![Box::new(FsLayer::with_root_path(root_path))]
		}
	}
	
	pub fn handle_input(&mut self, key_binding: KeyBinding) -> ActionResult {
		self.layers.last_mut().map(|layer| layer.handle_input(key_binding)).unwrap_or(ActionResult::Nothing)
	}
	
	pub fn render(&mut self, frame: &mut F) {
		for layer in self.layers.iter_mut() {
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
