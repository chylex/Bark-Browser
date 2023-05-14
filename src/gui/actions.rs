use std::collections::HashMap;
use std::io;

use crossterm::event::{Event, KeyCode};

use crate::gui::action::{Action, ActionResult, ExpandCollapse, Quit};
use crate::gui::action::movement::{MoveDown, MoveOrTraverseUpParent, MoveUp};
use crate::state::State;

pub struct ActionMap {
	keybinds: HashMap<KeyCode, Box<dyn Action>>,
}

impl ActionMap {
	pub fn new() -> Self {
		let mut me = Self { keybinds: HashMap::new() };
		me.add_char_mapping(' ', ExpandCollapse);
		me.add_char_mapping('h', MoveOrTraverseUpParent);
		me.add_char_mapping('j', MoveDown);
		me.add_char_mapping('k', MoveUp);
		me.add_char_mapping('q', Quit);
		me.add_key_mapping(KeyCode::Down, MoveDown);
		me.add_key_mapping(KeyCode::Left, MoveOrTraverseUpParent);
		me.add_key_mapping(KeyCode::Up, MoveUp);
		me
	}
	
	fn add_key_mapping<T: Action + 'static>(&mut self, code: KeyCode, action: T) {
		self.keybinds.insert(code, Box::new(action));
	}
	
	fn add_char_mapping<T: Action + 'static>(&mut self, c: char, action: T) {
		self.add_key_mapping(KeyCode::Char(c), action);
	}
	
	pub fn handle_next_action(&self, state: &mut State) -> Result<ActionResult, io::Error> {
		let event = crossterm::event::read()?;
		
		if let Event::Key(key) = event {
			if let Some(action) = self.keybinds.get(&key.code) {
				return Ok(action.perform(state));
			}
		}
		
		Ok(ActionResult::Nothing)
	}
}
