use ratatui::buffer::Buffer;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

use crate::component::filesystem::render::column;
use crate::file::{FileEntry, FileKind};

pub fn print(buf: &mut Buffer, x: u16, y: u16, entry: &FileEntry, level: usize, column_width: u16, is_selected: bool) {
	column::print_fixed_width_cell(buf, x, y, column_width, vec![
		Span::styled(" ".repeat(level), Style::default()),
		Span::styled(entry.name().str(), get_style(entry, is_selected)),
	]);
}

fn get_style(entry: &FileEntry, is_selected: bool) -> Style {
	let style = Style::default().fg(get_color(entry));
	
	if is_selected {
		style.add_modifier(Modifier::REVERSED)
	} else {
		style
	}
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
		
		FileKind::Unknown => Color::White,
	}
}
