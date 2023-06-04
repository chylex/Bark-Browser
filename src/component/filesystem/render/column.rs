use ratatui::buffer::Buffer;
use ratatui::style::Color;
use ratatui::text::{Line, Span};

pub fn print_fixed_width_cell(buf: &mut Buffer, x: u16, y: u16, column_width: u16, contents: Vec<Span>) {
	let (x2, _) = buf.set_line(x, y, &Line::from(contents), column_width.saturating_add(1));
	
	if x2 > x + column_width {
		if column_width > 0 {
			buf.get_mut(x + column_width - 1, y).set_char('~').set_fg(Color::DarkGray);
		}
		buf.get_mut(x + column_width, y).reset();
	}
}
