use std::io;

use crossterm::cursor;
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};

use crate::file::FileName;
use crate::gui::view::{R, SingleFrame};

pub fn print(frame: &mut SingleFrame, name: &FileName, level: usize, max_width: usize, is_selected: bool) -> R {
	let name_width = name.len() + level;
	let truncate_at = get_truncation_position(frame, name_width, max_width)?;
	
	frame.queue(ResetColor)?;
	frame.queue(Print(" ".repeat(level)))?;
	
	if is_selected {
		frame.queue(SetForegroundColor(Color::Black))?;
		frame.queue(SetBackgroundColor(Color::White))?;
	} else {
		frame.queue(SetForegroundColor(Color::White))?;
		frame.queue(SetBackgroundColor(Color::Black))?;
	}
	
	frame.queue(Print(name))?;
	
	if let Some((x, y)) = truncate_at {
		frame.queue(Print(name))?;
		frame.queue(MoveTo(x, y))?;
		frame.queue(SetForegroundColor(Color::DarkGrey))?;
		frame.queue(Print("~"))?;
	} else {
		frame.queue(ResetColor)?;
		frame.queue(Print(" ".repeat(max_width - name_width)))?;
	}
	
	Ok(())
}

fn get_truncation_position(frame: &mut SingleFrame, name_width: usize, max_width: usize) -> Result<Option<(u16, u16)>, io::Error> {
	if name_width <= max_width {
		return Ok(None);
	}
	
	frame.flush()?;
	let (initial_column, initial_row) = cursor::position()?;
	
	let last_char_column = if let Ok(max_width) = u16::try_from(max_width) {
		initial_column + max_width.saturating_sub(1)
	} else {
		initial_column
	};
	
	Ok(Some((last_char_column, initial_row)))
}
