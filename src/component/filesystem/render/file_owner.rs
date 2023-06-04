use ratatui::buffer::Buffer;
use ratatui::style::{Color, Style};
use ratatui::text::Span;

use crate::component::filesystem::render::column;
use crate::file::FileOwnerName;

pub fn print(buf: &mut Buffer, x: u16, y: u16, name: &FileOwnerName, column_width: u16) {
	let style = match name {
		FileOwnerName::Named(_)   => Style::default(),
		FileOwnerName::Numeric(_) => Style::default().fg(Color::Indexed(248 /* Grey66 */)),
		FileOwnerName::Unknown    => Style::default().fg(Color::DarkGray),
	};
	
	column::print_fixed_width_cell(buf, x, y, column_width, vec![
		Span::styled(name, style),
	]);
}
