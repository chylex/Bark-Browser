use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::ExitCode;

use crossterm::event::{Event, KeyEventKind};

use crate::state::action::{ActionResult, KeyBinding};
use crate::state::State;
use crate::state::view::View;

mod component;
mod file;
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
	
	let mut view = View::stdout()?;
	let mut state = State::with_root_path(&path.unwrap());
	
	'render: loop {
		view.render(|frame| state.render(frame))?;
		
		'event: loop {
			match handle_event(&mut state, crossterm::event::read()?) {
				ActionResult::Nothing => {
					continue 'event;
				}
				
				ActionResult::Redraw => {
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
	} else if let Event::Resize(_, _) = event {
		ActionResult::Redraw
	} else {
		ActionResult::Nothing
	}
}
