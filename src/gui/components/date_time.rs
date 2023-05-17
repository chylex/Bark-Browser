use std::time::SystemTime;

use chrono::{Datelike, DateTime, Local, Timelike, Utc};
use crossterm::style::{Print, ResetColor};

use crate::gui::view::{R, SingleFrame};

// Month + Space + Day + Space + Hour + Colon + Minute
pub const COLUMN_WIDTH: usize = 3 + 1 + 2 + 1 + 2 + 1 + 2;

// Replaces: Hour (2) + Colon (1) + Minute (2)
const YEAR_PADDED_WIDTH: usize = 5;

const MONTHS: &[&str] = &[
	"Jan", "Feb", "Mar",
	"Apr", "May", "Jun",
	"Jul", "Aug", "Sep",
	"Oct", "Nov", "Dec",
];

pub fn print(frame: &mut SingleFrame, system_time: Option<&SystemTime>) -> R {
	frame.queue(ResetColor)?;
	
	if let Some(system_time) = system_time {
		let date_time = DateTime::<Utc>::from(*system_time).naive_local();
		
		print_month(frame, date_time.month0() as usize)?;
		frame.queue(Print(" "))?;
		
		print_day_padded(frame, date_time.day())?;
		frame.queue(Print(" "))?;
		
		let year = date_time.year();
		if year == Local::now().year() { // TODO cache
			print_hour_minute_padded(frame, date_time.hour())?;
			frame.queue(Print(":"))?;
			print_hour_minute_padded(frame, date_time.minute())?;
		} else {
			print_year_padded(frame, year)?;
		}
	} else {
		frame.queue(Print("??? ?? ??:??"))?;
	}
	
	Ok(())
}

fn print_year_padded(frame: &mut SingleFrame, year: i32) -> R {
	frame.queue(Print(year))?;
	
	let year_digits = integer_len(year);
	if year_digits < YEAR_PADDED_WIDTH {
		frame.queue(Print(" ".repeat(YEAR_PADDED_WIDTH - year_digits)))?;
	}
	
	Ok(())
}

fn print_month(frame: &mut SingleFrame, month_index: usize) -> R {
	frame.queue(Print(MONTHS.get(month_index).unwrap_or(&"???")))
}

fn print_day_padded(frame: &mut SingleFrame, day: u32) -> R {
	if day < 10 {
		frame.queue(Print(" "))?;
	}
	
	frame.queue(Print(day))
}

fn print_hour_minute_padded(frame: &mut SingleFrame, value: u32) -> R {
	if value < 10 {
		frame.queue(Print("0"))?;
	}
	
	frame.queue(Print(value))
}

fn integer_len(n: i32) -> usize {
	let digit_count = n.abs().checked_ilog10().unwrap_or(1) + 1;
	let sign_len = if n < 0 { 1 } else { 0 };
	(digit_count + sign_len) as usize
}
