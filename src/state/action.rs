use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::state::layer::Layer;

pub trait Action<L> {
	fn perform(&self, layer: &mut L) -> ActionResult;
}

pub enum ActionResult {
	Nothing,
	Redraw,
	PushLayer(Box<dyn Layer>),
	ReplaceLayer(Box<dyn Layer>),
	PopLayer,
}

pub struct ActionMap<L> {
	keybinds: HashMap<KeyBinding, Box<dyn Action<L> + Sync>>,
}

impl<L> ActionMap<L> {
	pub fn new() -> Self {
		Self { keybinds: HashMap::new() }
	}
	
	pub fn add_key_mapping<T: Action<L> + Sync + 'static>(&mut self, code: KeyCode, modifiers: KeyModifiers, action: T) {
		self.keybinds.insert(KeyBinding::Code(code, modifiers), Box::new(action));
	}
	
	pub fn add_char_mapping<T: Action<L> + Sync + 'static>(&mut self, c: char, action: T) {
		self.keybinds.insert(KeyBinding::Char(c), Box::new(action));
	}
	
	pub fn handle_action(&self, layer: &mut L, key_binding: KeyBinding) -> ActionResult {
		self.keybinds.get(&key_binding).map(|action| action.perform(layer)).unwrap_or(ActionResult::Nothing)
	}
}

#[derive(Eq, PartialEq, Hash)]
pub enum KeyBinding {
	Char(char),
	Code(KeyCode, KeyModifiers),
}

impl From<KeyEvent> for KeyBinding {
	fn from(key_event: KeyEvent) -> Self {
		if let KeyCode::Char(char) = key_event.code {
			KeyBinding::Char(char)
		} else {
			KeyBinding::Code(key_event.code, key_event.modifiers)
		}
	}
}
