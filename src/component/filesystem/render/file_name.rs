use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};

use crate::component::filesystem::render::column;
use crate::file::{FileEntry, FileKind};
use crate::state::view::{R, View};

pub fn print(view: &mut View, entry: &FileEntry, level: usize, column_width: usize, is_selected: bool) -> R {
	column::print_fixed_width_cell(view, |view| {
		view.queue(ResetColor)?;
		view.queue(Print(" ".repeat(level)))?;
		
		if is_selected {
			view.queue(SetForegroundColor(Color::Black))?;
			view.queue(SetBackgroundColor(get_color(entry)))?;
		} else {
			view.queue(SetForegroundColor(get_color(entry)))?;
			view.queue(SetBackgroundColor(Color::Black))?;
		}
		
		view.queue(Print(entry.name()))
	}, column_width)
}

fn get_color(entry: &FileEntry) -> Color {
	match entry.kind() {
		FileKind::File { .. } => {
			if entry.mode().is_executable_by_any() == Some(true) {
				Color::Green
			} else {
				Color::White
			}
		},
		
		FileKind::Directory => Color::Blue,
		FileKind::Symlink => Color::Cyan,
		FileKind::Socket => Color::Magenta,
		
		FileKind::BlockDevice |
		FileKind::CharDevice |
		FileKind::Pipe => Color::Yellow,
		
		_ => Color::White,
	}
}
