use std::env;
use std::error::Error;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use crossterm::{cursor, QueueableCommand, terminal};

use crate::file::FileOwnerNameCache;
use crate::gui::ActionMap;
use crate::gui::view::View;
use crate::state::action::ActionResult;
use crate::state::State;

mod file;
mod gui;
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
	
	terminal::enable_raw_mode()?;
	stdout().queue(terminal::EnterAlternateScreen)?;
	stdout().queue(cursor::Hide)?;
	stdout().flush()?;
	
	let mut view = View::stdout();
	let actions = ActionMap::new();
	
	let mut state = State::with_root_path(&path.unwrap());
	state.tree.expand(state.tree.view.root_id());
	
	let mut file_owner_name_cache = FileOwnerNameCache::new();
	
	'render: loop {
		view.render_state(&state, &mut file_owner_name_cache)?;
		
		'action: loop {
			match actions.handle_next_action(&mut state)? {
				ActionResult::Nothing => {
					continue 'action;
				}
				
				ActionResult::Redraw { tree_structure_changed } => {
					if tree_structure_changed {
						view.tree_structure_changed();
					}
					
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
