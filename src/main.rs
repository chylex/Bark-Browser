use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::ExitCode;

use crossterm::event::{Event, KeyEventKind};

use input::keymap::KeyBinding;

use crate::state::action::ActionResult;
use crate::state::Environment;
use crate::state::State;
use crate::state::view::View;

mod component;
mod file;
mod input;
mod state;
mod util;

fn main() -> Result<ExitCode, Box<dyn Error>> {
	let args = env::args_os().skip(1).collect::<Vec<_>>();
	if args.len() > 1 {
		println!("Too many arguments!");
		return Ok(ExitCode::SUCCESS);
	}
	
	let path = args.get(0).map(PathBuf::from).or_else(|| env::current_dir().ok());
	if path.is_none() {
		println!("Invalid path!");
		return Ok(ExitCode::FAILURE);
	}
	
	View::restore_terminal_on_panic();
	let mut view = View::stdout()?;
	
	let environment = Environment::try_from(&view)?;
	let mut state = State::with_root_path(&path.unwrap(), environment);
	
	'render: loop {
		view.render(|frame| state.render(frame))?;
		
		'event: loop {
			match handle_event(&mut state, crossterm::event::read()?) {
				ActionResult::Nothing => {
					continue 'event;
				}
				
				ActionResult::Draw => {
					continue 'render;
				}
				
				ActionResult::Redraw => {
					view.clear()?;
					continue 'render;
				}
				
				ActionResult::PushLayer(layer) => {
					state.push_layer(layer);
					continue 'render;
				}
				
				ActionResult::ReplaceLayer(layer) => {
					state.pop_layer();
					state.push_layer(layer);
					continue 'render;
				}
				
				ActionResult::PopLayer => {
					if state.pop_layer() {
						break 'render;
					} else {
						continue 'render;
					}
				}
			}
		}
	}
	
	view.close()?;
	
	Ok(ExitCode::SUCCESS)
}

fn handle_event(state: &mut State, event: Event) -> ActionResult {
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
