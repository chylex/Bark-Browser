use std::io;

use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::state::State;

pub fn handle_next_action(state: &mut State) -> Result<ActionResult, io::Error> {
	let event = crossterm::event::read()?;
	
	if let Event::Key(key) = event {
		let result = handle_key(state, key);
		return Ok(result);
	}
	
	Ok(ActionResult::Nothing)
}

fn handle_key(state: &mut State, key: KeyEvent) -> ActionResult {
	let code = key.code;
	
	if let KeyCode::Char(char) = code {
		if char == 'q' {
			return ActionResult::Quit;
		}
	}
	
	ActionResult::Nothing
}

pub enum ActionResult {
	Nothing,
	Redraw,
	Quit,
}
