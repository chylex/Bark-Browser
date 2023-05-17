use crossterm::style::{Color, ContentStyle, Print, ResetColor, Stylize};

use crate::file::{FileKind, FileMode, Permission};
use crate::gui::view::{R, SingleFrame};

// Kind + Owner + Group + Other
pub const COLUMN_WIDTH: usize = 1 + 3 + 3 + 3;

const READ_BIT_COLOR: Color = Color::DarkBlue;
const WRITE_BIT_COLOR: Color = Color::DarkRed;
const EXECUTE_BIT_COLOR: Color = Color::DarkGreen;

pub fn print(frame: &mut SingleFrame, kind: &FileKind, mode: &FileMode) -> R {
	frame.queue(ResetColor)?;
	
	print_kind(frame, kind)?;
	
	let user = mode.user();
	let group = mode.group();
	let others = mode.others();
	
	print_permission(frame, user.read(), 'r', READ_BIT_COLOR)?;
	print_permission(frame, user.write(), 'w', WRITE_BIT_COLOR)?;
	print_permission_or_special(frame, user.execute(), mode.is_setuid(), 'x', 'S', 's', EXECUTE_BIT_COLOR)?;
	
	print_permission(frame, group.read(), 'r', READ_BIT_COLOR)?;
	print_permission(frame, group.write(), 'w', WRITE_BIT_COLOR)?;
	print_permission_or_special(frame, group.execute(), mode.is_setgid(), 'x', 'S', 's', EXECUTE_BIT_COLOR)?;
	
	print_permission(frame, others.read(), 'r', READ_BIT_COLOR)?;
	print_permission(frame, others.write(), 'w', WRITE_BIT_COLOR)?;
	print_permission_or_special(frame, others.execute(), mode.is_sticky(), 'x', 'T', 't', EXECUTE_BIT_COLOR)?;
	
	Ok(())
}

fn print_kind(frame: &mut SingleFrame, kind: &FileKind) -> R {
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
	
	print_char(frame, c, Color::Grey)
}

fn print_permission(frame: &mut SingleFrame, permission: Permission, c: char, color: Color) -> R {
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
	
	print_char(frame, c, color)
}

fn print_permission_or_special(frame: &mut SingleFrame, permission: Permission, special: Option<bool>, permission_only_char: char, special_only_char: char, permission_and_special_char: char, color: Color) -> R {
	if special == Some(true) {
		let char = if permission == Permission::Yes { permission_and_special_char } else { special_only_char };
		print_char(frame, char, color)
	} else {
		print_permission(frame, permission, permission_only_char, color)
	}
}

fn print_char(frame: &mut SingleFrame, char: char, color: Color) -> R {
	frame.queue(Print(ContentStyle::new().with(color).apply(char)))
}
