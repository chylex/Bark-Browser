use crossterm::style::{Color, ContentStyle, Print, ResetColor, Stylize};

use crate::file::{FileKind, FileMode, Permission};
use crate::gui::view::{R, View};

// Kind + Owner + Group + Other
pub const COLUMN_WIDTH: usize = 1 + 3 + 3 + 3;

const READ_BIT_COLOR: Color = Color::DarkBlue;
const WRITE_BIT_COLOR: Color = Color::DarkRed;
const EXECUTE_BIT_COLOR: Color = Color::DarkGreen;

pub fn print(view: &mut View, kind: &FileKind, mode: &FileMode) -> R {
	view.queue(ResetColor)?;
	
	print_kind(view, kind)?;
	
	let user = mode.user();
	let group = mode.group();
	let others = mode.others();
	
	print_permission(view, user.read(), 'r', READ_BIT_COLOR)?;
	print_permission(view, user.write(), 'w', WRITE_BIT_COLOR)?;
	print_permission_or_special(view, user.execute(), mode.is_setuid(), 'x', 'S', 's', EXECUTE_BIT_COLOR)?;
	
	print_permission(view, group.read(), 'r', READ_BIT_COLOR)?;
	print_permission(view, group.write(), 'w', WRITE_BIT_COLOR)?;
	print_permission_or_special(view, group.execute(), mode.is_setgid(), 'x', 'S', 's', EXECUTE_BIT_COLOR)?;
	
	print_permission(view, others.read(), 'r', READ_BIT_COLOR)?;
	print_permission(view, others.write(), 'w', WRITE_BIT_COLOR)?;
	print_permission_or_special(view, others.execute(), mode.is_sticky(), 'x', 'T', 't', EXECUTE_BIT_COLOR)?;
	
	Ok(())
}

fn print_kind(view: &mut View, kind: &FileKind) -> R {
	let c = match kind {
		FileKind::File { size: _ } => { '-' }
		FileKind::Directory => { 'd' }
		FileKind::Symlink => { 'l' }
		FileKind::BlockDevice => { 'b' }
		FileKind::CharDevice => { 'c' }
		FileKind::Pipe => { 'p' }
		FileKind::Socket => { 's' }
		FileKind::Unknown => { '?' }
	};
	
	print_char(view, c, Color::Grey)
}

fn print_permission(view: &mut View, permission: Permission, c: char, color: Color) -> R {
	let (c, color) = match permission {
		Permission::Yes => {
			(c, color)
		}
		Permission::No => {
			('-', Color::Grey)
		}
		Permission::Unknown => {
			('?', Color::DarkGrey)
		}
	};
	
	print_char(view, c, color)
}

fn print_permission_or_special(view: &mut View, permission: Permission, special: Option<bool>, permission_only_char: char, special_only_char: char, permission_and_special_char: char, color: Color) -> R {
	if special == Some(true) {
		let char = if permission == Permission::Yes { permission_and_special_char } else { special_only_char };
		print_char(view, char, color)
	} else {
		print_permission(view, permission, permission_only_char, color)
	}
}

fn print_char(view: &mut View, char: char, color: Color) -> R {
	view.queue(Print(ContentStyle::new().with(color).apply(char)))
}
