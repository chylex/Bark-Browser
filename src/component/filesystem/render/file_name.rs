use ratatui::buffer::Buffer;
use ratatui::style::{Color, Style};
use ratatui::text::Span;

use crate::component::filesystem::render::column;
use crate::file::{FileEntry, FileKind};

pub fn print(buf: &mut Buffer, x: u16, y: u16, entry: &FileEntry, level: usize, column_width: u16, is_selected: bool) {
	let style = if is_selected {
		Style::default().fg(Color::Black).bg(get_color(entry))
	} else {
		Style::default().fg(get_color(entry)).bg(Color::Black)
	};
	
	column::print_fixed_width_cell(buf, x, y, column_width, vec![
		Span::styled(" ".repeat(level), Style::default()),
		Span::styled(entry.name().str(), style),
	])
}

fn get_color(entry: &FileEntry) -> Color {
	match entry.kind() {
		FileKind::File { .. } => {
			if entry.mode().is_executable_by_any() == Some(true) {
				Color::LightGreen
			} else {
				Color::White
			}
		}
		
		FileKind::Directory => Color::LightBlue,
		FileKind::Symlink => Color::LightCyan,
		FileKind::Socket => Color::LightMagenta,
		
		FileKind::BlockDevice |
		FileKind::CharDevice |
		FileKind::Pipe => Color::LightYellow,
		
		_ => Color::White,
	}
}
