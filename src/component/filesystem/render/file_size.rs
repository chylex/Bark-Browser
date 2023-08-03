#![allow(clippy::arithmetic_side_effects)]

use ratatui::buffer::Buffer;
use ratatui::style::Style;

const VALUE_MAX_WIDTH: u16 = 3;
pub const COLUMN_WIDTH: u16 = VALUE_MAX_WIDTH + 1 + 2;

const UNITS: &[&str] = &[
	"B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"
];

pub fn print(buf: &mut Buffer, x: u16, y: u16, size: Option<u64>) {
	if let Some(size) = size {
		print_size_with_auto_unit(buf, x, y, size);
	}
}

fn print_size_with_auto_unit(buf: &mut Buffer, x: u16, y: u16, size: u64) {
	let mut size = size;
	let mut unit = 0;
	
	while size >= 1_000_000 && unit < UNITS.len() - 1 {
		size /= 1_000;
		unit += 1;
	}
	
	if size < 1_000 || unit == UNITS.len() - 1 {
		#[allow(clippy::indexing_slicing)] // Guarded by previous loop.
		print_size_with_unit(buf, x, y, size.to_string(), UNITS[unit]);
		return;
	}
	
	#[allow(clippy::indexing_slicing)] // Guarded by previous condition.
	let unit_symbol = UNITS[unit + 1];
	
	if size >= 10_000 || (size / 1_000) * 1_000 == size {
		size /= 1_000;
		print_size_with_unit(buf, x, y, size.to_string(), unit_symbol);
	} else {
		let whole_part = size / 1_000;
		let decimal_part = (size % 1_000) / 100;
		print_size_with_unit(buf, x, y, format!("{whole_part}.{decimal_part}"), unit_symbol);
	}
}

fn print_size_with_unit(buf: &mut Buffer, x: u16, y: u16, size_text: String, unit_symbol: &str) {
	let symbol_width = unit_symbol.len();
	let total_width = size_text.len() + 1 + symbol_width;
	
	#[allow(clippy::cast_possible_truncation)] // Widths are always small enough.
	buf.set_string(x + COLUMN_WIDTH - total_width as u16, y, size_text, Style::default());
	
	#[allow(clippy::cast_possible_truncation)] // Widths are always small enough.
	buf.set_string(x + COLUMN_WIDTH - symbol_width as u16, y, unit_symbol, Style::default());
}
