use std::collections::HashMap;
use std::io;

use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::gui::action::{Action, ActionResult, ExpandCollapse, Quit};
use crate::gui::action::movement::{MoveDown, MoveOrTraverseUpParent, MoveToNextSibling, MoveToPreviousSibling, MoveUp};
use crate::state::State;

pub struct ActionMap {
	keybinds: HashMap<KeyBinding, Box<dyn Action>>,
}

impl ActionMap {
	pub fn new() -> Self {
		let mut me = Self { keybinds: HashMap::new() };
		me.add_char_mapping(' ', ExpandCollapse);
		me.add_char_mapping('J', MoveToNextSibling);
		me.add_char_mapping('K', MoveToPreviousSibling);
		me.add_char_mapping('h', MoveOrTraverseUpParent);
		me.add_char_mapping('j', MoveDown);
		me.add_char_mapping('k', MoveUp);
		me.add_char_mapping('q', Quit);
		me.add_key_mapping(KeyCode::Down, KeyModifiers::ALT, MoveToNextSibling);
		me.add_key_mapping(KeyCode::Down, KeyModifiers::NONE, MoveDown);
		me.add_key_mapping(KeyCode::Left, KeyModifiers::NONE, MoveOrTraverseUpParent);
		me.add_key_mapping(KeyCode::Up, KeyModifiers::ALT, MoveToPreviousSibling);
		me.add_key_mapping(KeyCode::Up, KeyModifiers::NONE, MoveUp);
		me
	}
	
	fn add_key_mapping<T: Action + 'static>(&mut self, code: KeyCode, modifiers: KeyModifiers, action: T) {
		self.keybinds.insert(KeyBinding::Code(code, modifiers), Box::new(action));
	}
	
	fn add_char_mapping<T: Action + 'static>(&mut self, c: char, action: T) {
		self.keybinds.insert(KeyBinding::Char(c), Box::new(action));
	}
	
	pub fn handle_next_action(&self, state: &mut State) -> Result<ActionResult, io::Error> {
		let event = crossterm::event::read()?;
		
		if let Event::Key(key) = event {
			let key_binding = if let KeyCode::Char(char) = key.code {
				KeyBinding::Char(char)
			} else {
				KeyBinding::Code(key.code, key.modifiers)
			};
			
			if let Some(action) = self.keybinds.get(&key_binding) {
				return Ok(action.perform(state));
			}
		}
		
		Ok(ActionResult::Nothing)
	}
}

#[derive(Eq, PartialEq, Hash)]
enum KeyBinding {
	Char(char),
	Code(KeyCode, KeyModifiers),
}
