use std::error::Error;
use std::io::{stdout, Write};
use std::path::Path;

use crossterm::{cursor, QueueableCommand, terminal};

use crate::gui::actions::{ActionResult, handle_next_action};
use crate::gui::view::View;
use crate::state::State;

mod file;
mod gui;
mod state;

fn main() -> Result<(), Box<dyn Error>> {
	terminal::enable_raw_mode()?;
	stdout().queue(terminal::EnterAlternateScreen)?;
	stdout().queue(cursor::Hide)?;
	stdout().flush()?;
	
	let mut view = View::stdout();
	
	let mut state = State::with_root_path(Path::new("/"));
	state.tree.expand(state.tree.root_id);
	
	'render: loop {
		view.render_state(&state)?;
		
		loop {
			match handle_next_action(&mut state)? {
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
	
	Ok(())
}
