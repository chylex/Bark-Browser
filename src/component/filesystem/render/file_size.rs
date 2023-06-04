use ratatui::buffer::Buffer;
use ratatui::style::Style;

use crate::util::int_len;

const VALUE_MAX_WIDTH: u16 = 3;
pub const COLUMN_WIDTH: u16 = VALUE_MAX_WIDTH + 1 + 2;

const UNITS: &[&str] = &[
	"B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"
];

pub fn print(buf: &mut Buffer, x: u16, y: u16, size: Option<u64>) {
	if let Some(size) = size {
		print_size_with_unit(buf, x, y, size)
	}
}

fn print_size_with_unit(buf: &mut Buffer, x: u16, y: u16, size: u64) {
	let mut size = size;
	let mut unit = 0;
	
	while size >= 1000 && unit < UNITS.len() - 1 {
		size /= 1000;
		unit += 1;
	}
	
	let unit_symbol = UNITS[unit];
	let symbol_width = unit_symbol.len();
	let total_width = int_len(size) + 1 + symbol_width;
	
	buf.set_string(x + COLUMN_WIDTH - total_width as u16, y, size.to_string(), Style::default());
	buf.set_string(x + COLUMN_WIDTH - symbol_width as u16, y, unit_symbol, Style::default());
}
