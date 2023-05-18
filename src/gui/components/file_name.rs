use std::io;

use crossterm::cursor;
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};

use crate::file::FileName;
use crate::gui::view::{R, View};

pub fn print(view: &mut View, name: &FileName, level: usize, max_width: usize, is_selected: bool) -> R {
	let name_width = name.len() + level;
	let truncate_at = get_truncation_position(view, name_width, max_width)?;
	
	view.queue(ResetColor)?;
	view.queue(Print(" ".repeat(level)))?;
	
	if is_selected {
		view.queue(SetForegroundColor(Color::Black))?;
		view.queue(SetBackgroundColor(Color::White))?;
	} else {
		view.queue(SetForegroundColor(Color::White))?;
		view.queue(SetBackgroundColor(Color::Black))?;
	}
	
	view.queue(Print(name))?;
	
	if let Some((x, y)) = truncate_at {
		view.queue(MoveTo(x, y))?;
		view.queue(SetForegroundColor(Color::DarkGrey))?;
		view.queue(Print("~"))?;
	} else {
		view.queue(ResetColor)?;
		view.queue(Print(" ".repeat(max_width - name_width)))?;
	}
	
	Ok(())
}

fn get_truncation_position(view: &mut View, name_width: usize, max_width: usize) -> Result<Option<(u16, u16)>, io::Error> {
	if name_width <= max_width {
		return Ok(None);
	}
	
	view.flush()?;
	let (initial_column, initial_row) = cursor::position()?;
	
	let last_char_column = if let Ok(max_width) = u16::try_from(max_width) {
		initial_column + max_width.saturating_sub(1)
	} else {
		initial_column
	};
	
	Ok(Some((last_char_column, initial_row)))
}
