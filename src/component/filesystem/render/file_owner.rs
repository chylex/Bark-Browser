use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};

use crate::component::filesystem::render::column;
use crate::file::FileOwnerName;
use crate::state::view::{R, View};

pub fn print(view: &mut View, name: &FileOwnerName, column_width: usize) -> R {
	column::print_fixed_width_cell(view, |view| {
		view.queue(ResetColor)?;
		
		match name {
			FileOwnerName::Named(_) => {},
			FileOwnerName::Numeric(_) => view.queue(SetForegroundColor(Color::AnsiValue(248 /* Grey66 */)))?,
			FileOwnerName::Unknown => view.queue(SetForegroundColor(Color::DarkGrey))?,
		}
		
		view.queue(Print(name))
	}, column_width)
}
