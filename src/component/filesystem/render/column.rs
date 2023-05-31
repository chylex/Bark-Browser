use std::io;

use crossterm::cursor;
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};

use crate::state::view::{R, View};

pub fn print_fixed_width_cell<F>(view: &mut View, print_contents: F, column_width: usize) -> R where F: FnOnce(&mut View) -> R {
	let (cursor_initial_column, cursor_initial_row) = get_cursor_position(view)?;
	let next_cell_start_column = get_next_cell_start_column(cursor_initial_column, column_width);
	
	print_contents(view)?;
	
	let (cursor_final_column, cursor_final_row) = get_cursor_position(view)?;
	
	if cursor_final_row == cursor_initial_row && cursor_final_column <= next_cell_start_column {
		view.queue(ResetColor)?;
		for _ in cursor_final_column..next_cell_start_column {
			view.queue(Print(" "))?;
		}
	} else {
		view.queue(MoveTo(next_cell_start_column.saturating_sub(1), cursor_initial_row))?;
		view.queue(SetForegroundColor(Color::DarkGrey))?;
		view.queue(Print("~"))?;
	}
	
	Ok(())
}

fn get_next_cell_start_column(start_column: u16, column_width: usize) -> u16 {
	if let Ok(column_width) = u16::try_from(column_width) {
		start_column.saturating_add(column_width)
	} else {
		u16::MAX
	}
}

fn get_cursor_position(view: &mut View) -> Result<(u16, u16), io::Error> {
	// Flushing before retrieving cursor position is necessary on Windows and possibly in other terminals.
	view.flush()?;
	
	cursor::position()
}
