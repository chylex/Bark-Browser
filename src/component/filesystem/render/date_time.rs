use std::time::SystemTime;

use chrono::{Datelike, DateTime, Local, Timelike, Utc};
use crossterm::style::{Print, ResetColor};

use crate::state::view::{R, View};
use crate::util;

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

pub fn print(view: &mut View, system_time: Option<&SystemTime>) -> R {
	view.queue(ResetColor)?;
	
	if let Some(system_time) = system_time {
		let date_time = DateTime::<Utc>::from(*system_time).naive_local();
		
		print_month(view, date_time.month0() as usize)?;
		view.queue(Print(" "))?;
		
		print_day_padded(view, date_time.day())?;
		view.queue(Print(" "))?;
		
		let year = date_time.year();
		if year == Local::now().year() { // TODO cache
			print_hour_minute_padded(view, date_time.hour())?;
			view.queue(Print(":"))?;
			print_hour_minute_padded(view, date_time.minute())?;
		} else {
			print_year_padded(view, year)?;
		}
	} else {
		view.queue(Print("??? ?? ??:??"))?;
	}
	
	Ok(())
}

fn print_year_padded(view: &mut View, year: i32) -> R {
	view.queue(Print(year))?;
	
	let year_digits = util::int_len(year);
	if year_digits < YEAR_PADDED_WIDTH {
		view.queue(Print(" ".repeat(YEAR_PADDED_WIDTH - year_digits)))?;
	}
	
	Ok(())
}

fn print_month(view: &mut View, month_index: usize) -> R {
	view.queue(Print(MONTHS.get(month_index).unwrap_or(&"???")))
}

fn print_day_padded(view: &mut View, day: u32) -> R {
	if day < 10 {
		view.queue(Print(" "))?;
	}
	
	view.queue(Print(day))
}

fn print_hour_minute_padded(view: &mut View, value: u32) -> R {
	if value < 10 {
		view.queue(Print("0"))?;
	}
	
	view.queue(Print(value))
}
