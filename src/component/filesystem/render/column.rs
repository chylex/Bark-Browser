use ratatui::buffer::Buffer;
use ratatui::style::Color;
use ratatui::text::{Line, Span};

pub fn print_fixed_width_cell(buf: &mut Buffer, x: u16, y: u16, column_width: u16, contents: Vec<Span>) {
	let requested_x2 = x.saturating_add(column_width);
	let (actual_x2, _) = buf.set_line(x, y, &Line::from(contents), column_width.saturating_add(1));
	
	if actual_x2 > requested_x2 {
		buf.get_mut(requested_x2.saturating_sub(1), y).set_char('~').set_fg(Color::DarkGray);
		buf.get_mut(requested_x2, y).reset();
	}
}
