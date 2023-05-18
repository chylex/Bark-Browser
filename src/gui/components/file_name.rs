use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};

use crate::file::FileName;
use crate::gui::components::column;
use crate::gui::view::{R, View};

pub fn print(view: &mut View, name: &FileName, level: usize, column_width: usize, is_selected: bool) -> R {
	column::print_fixed_width_cell(view, |view| {
		view.queue(ResetColor)?;
		view.queue(Print(" ".repeat(level)))?;
		
		if is_selected {
			view.queue(SetForegroundColor(Color::Black))?;
			view.queue(SetBackgroundColor(Color::White))?;
		} else {
			view.queue(SetForegroundColor(Color::White))?;
			view.queue(SetBackgroundColor(Color::Black))?;
		}
		
		view.queue(Print(name))
	}, column_width)
}
