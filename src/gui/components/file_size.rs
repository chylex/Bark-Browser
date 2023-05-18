use crossterm::style::{Print, ResetColor};

use crate::gui::view::{R, View};
use crate::util::int_len;

const VALUE_WIDTH: usize = 3;
pub const COLUMN_WIDTH: usize = VALUE_WIDTH + 1 + 2;

const UNITS: &[&str] = &[
	"B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"
];

pub fn print(view: &mut View, size: Option<u64>) -> R {
	view.queue(ResetColor)?;
	
	if let Some(size) = size {
		print_size_with_unit(view, size)
	} else {
		view.queue(Print(" ".repeat(COLUMN_WIDTH)))
	}
}

fn print_size_with_unit(view: &mut View, size: u64) -> R {
	let mut size = size;
	let mut unit = 0;
	
	while size >= 1000 && unit < UNITS.len() - 1 {
		size /= 1000;
		unit += 1;
	}
	
	let unit_symbol = UNITS[unit];
	let actual_width = int_len(size) + 1 + unit_symbol.len();
	if actual_width < COLUMN_WIDTH {
		view.queue(Print(" ".repeat(COLUMN_WIDTH - actual_width)))?;
	}
	
	view.queue(Print(size))?;
	view.queue(Print(" "))?;
	view.queue(Print(unit_symbol))?;
	
	Ok(())
}
