use std::path::Path;

use crossterm::event::{Event, KeyEventKind};

use crate::input::keymap::KeyBinding;
use crate::state::{Environment, State};
use crate::state::action::ActionResult;
use crate::state::event::EventResult;
use crate::state::view::View;

pub fn run(start_path: &Path) -> std::io::Result<()> {
	View::restore_terminal_on_panic();
	let mut view = View::stdout()?;
	
	let environment = Environment::try_from(&view)?;
	let mut state = State::with_root_path(start_path, environment);
	
	loop {
		match state.handle_events() {
			EventResult::Nothing => {}
			
			EventResult::Draw => {
				view.set_dirty(false);
			}
			
			EventResult::Redraw => {
				view.set_dirty(true);
			}
		}
		
		view.render(|frame| state.render(frame))?;
		
		match handle_terminal_event(&mut state, crossterm::event::read()?) {
			ActionResult::Nothing => {
				continue;
			}
			
			ActionResult::Draw => {
				view.set_dirty(false);
				continue;
			}
			
			ActionResult::Redraw => {
				view.set_dirty(true);
				continue;
			}
			
			ActionResult::PushLayer(layer) => {
				state.push_layer(layer);
				view.set_dirty(false);
				continue;
			}
			
			ActionResult::ReplaceLayer(layer) => {
				state.pop_layer();
				state.push_layer(layer);
				view.set_dirty(false);
				continue;
			}
			
			ActionResult::PopLayer => {
				if state.pop_layer() {
					break;
				} else {
					view.set_dirty(false);
					continue;
				}
			}
		}
	}
	
	view.close()
}

#[allow(clippy::needless_pass_by_value)]
fn handle_terminal_event(state: &mut State, event: Event) -> ActionResult {
	if let Event::Key(key) = event {
		if key.kind == KeyEventKind::Release {
			ActionResult::Nothing
		} else {
			state.handle_input(KeyBinding::from(key))
		}
	} else if let Event::Resize(w, h) = event {
		state.handle_resize(w, h);
		ActionResult::Draw
	} else {
		ActionResult::Nothing
	}
}
