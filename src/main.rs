use std::env;
use std::error::Error;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use crossterm::{cursor, QueueableCommand, terminal};

use crate::gui::action::ActionResult;
use crate::gui::ActionMap;
use crate::gui::view::View;
use crate::state::State;

mod file;
mod gui;
mod state;

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
	
	terminal::enable_raw_mode()?;
	stdout().queue(terminal::EnterAlternateScreen)?;
	stdout().queue(cursor::Hide)?;
	stdout().flush()?;
	
	let mut view = View::stdout();
	let actions = ActionMap::new();
	
	let mut state = State::with_root_path(&path.unwrap());
	state.tree.expand(state.tree.root_id);
	
	'render: loop {
		view.render_state(&state)?;
		
		loop {
			match actions.handle_next_action(&mut state)? {
				ActionResult::Nothing => {}
				ActionResult::Redraw => {
					continue 'render;
				}
				ActionResult::Quit => {
					break 'render;
				}
			}
		}
	}
	
	stdout().queue(cursor::Show)?;
	stdout().queue(terminal::LeaveAlternateScreen)?;
	stdout().flush()?;
	terminal::disable_raw_mode()?;
	
	Ok(ExitCode::SUCCESS)
}
