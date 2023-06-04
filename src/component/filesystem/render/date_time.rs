use std::time::SystemTime;

use chrono::{Datelike, DateTime, Local, Timelike, Utc};
use ratatui::buffer::Buffer;
use ratatui::style::Style;

// Month + Space + Day + Space + Hour + Colon + Minute
// Month + Space + Day + Space + Year + Space
pub const COLUMN_WIDTH: u16 = 3 + 1 + 2 + 1 + 2 + 1 + 2;

const MONTHS: &[&str] = &[
	"Jan", "Feb", "Mar",
	"Apr", "May", "Jun",
	"Jul", "Aug", "Sep",
	"Oct", "Nov", "Dec",
];

pub fn print(buf: &mut Buffer, x: u16, y: u16, system_time: Option<&SystemTime>) {
	if let Some(system_time) = system_time {
		let date_time = DateTime::<Utc>::from(*system_time).naive_local();
		
		print_month(buf, x, y, date_time.month0() as usize);
		print_day_padded(buf, x + 4, y, date_time.day());
		
		let year = date_time.year();
		if year == Local::now().year() { // TODO cache
			print_hour_minute(buf, x + 7, y, date_time.hour());
			buf.get_mut(x + 9, y).set_char(':');
			print_hour_minute(buf, x + 10, y, date_time.minute());
		} else {
			print_year(buf, x + 7, y, year);
		}
	} else {
		buf.set_string(x, y, "??? ?? ??:??", Style::default());
	}
}

fn print_year(buf: &mut Buffer, x: u16, y: u16, year: i32) {
	buf.set_string(x, y, year.to_string(), Style::default());
}

fn print_month(buf: &mut Buffer, x: u16, y: u16, month_index: usize) {
	buf.set_string(x, y, MONTHS.get(month_index).unwrap_or(&"???"), Style::default());
}

fn print_day_padded(buf: &mut Buffer, x: u16, y: u16, day: u32) {
	buf.set_string(if day < 10 { x + 1 } else { x }, y, day.to_string(), Style::default());
}

fn print_hour_minute(buf: &mut Buffer, x: u16, y: u16, value: u32) {
	if let Some(single_digit) = char::from_digit(value, 10) {
		buf.get_mut(x, y).set_char('0');
		buf.get_mut(x + 1, y).set_char(single_digit);
	} else {
		buf.set_string(x, y, value.to_string(), Style::reset());
	}
}
