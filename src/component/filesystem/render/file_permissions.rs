#![allow(clippy::arithmetic_side_effects)]

use ratatui::buffer::Buffer;
use ratatui::style::Color;

use crate::file::{FileKind, FileMode, Permission};

// Kind + Owner + Group + Other
pub const COLUMN_WIDTH: u16 = 1 + 3 + 3 + 3;

const READ_BIT_COLOR: Color = Color::LightBlue;
const WRITE_BIT_COLOR: Color = Color::LightRed;
const EXECUTE_BIT_COLOR: Color = Color::LightGreen;

pub fn print(buf: &mut Buffer, x: u16, y: u16, kind: &FileKind, mode: FileMode) {
	print_kind(buf, x, y, kind);
	
	let user = mode.user();
	let group = mode.group();
	let others = mode.others();
	
	print_permission(buf, x + 1, y, user.read(), 'r', READ_BIT_COLOR);
	print_permission(buf, x + 2, y, user.write(), 'w', WRITE_BIT_COLOR);
	print_permission_or_special(buf, x + 3, y, user.execute(), mode.is_setuid(), 'x', 'S', 's', EXECUTE_BIT_COLOR);
	
	print_permission(buf, x + 4, y, group.read(), 'r', READ_BIT_COLOR);
	print_permission(buf, x + 5, y, group.write(), 'w', WRITE_BIT_COLOR);
	print_permission_or_special(buf, x + 6, y, group.execute(), mode.is_setgid(), 'x', 'S', 's', EXECUTE_BIT_COLOR);
	
	print_permission(buf, x + 7, y, others.read(), 'r', READ_BIT_COLOR);
	print_permission(buf, x + 8, y, others.write(), 'w', WRITE_BIT_COLOR);
	print_permission_or_special(buf, x + 9, y, others.execute(), mode.is_sticky(), 'x', 'T', 't', EXECUTE_BIT_COLOR);
}

fn print_kind(buf: &mut Buffer, x: u16, y: u16, kind: &FileKind) {
	let c = match kind {
		FileKind::File { .. } => { '-' }
		FileKind::Directory   => { 'd' }
		FileKind::Symlink     => { 'l' }
		FileKind::BlockDevice => { 'b' }
		FileKind::CharDevice  => { 'c' }
		FileKind::Pipe        => { 'p' }
		FileKind::Socket      => { 's' }
		FileKind::Unknown     => { '?' }
	};
	
	print_char(buf, x, y, c, Color::Gray);
}

fn print_permission(buf: &mut Buffer, x: u16, y: u16, permission: Permission, c: char, color: Color) {
	let (c, color) = match permission {
		Permission::Yes => {
			(c, color)
		}
		Permission::No => {
			('-', Color::Gray)
		}
		Permission::Unknown => {
			('?', Color::DarkGray)
		}
	};
	
	print_char(buf, x, y, c, color);
}

fn print_permission_or_special(buf: &mut Buffer, x: u16, y: u16, permission: Permission, special: Option<bool>, permission_only_char: char, special_only_char: char, permission_and_special_char: char, color: Color) {
	if special == Some(true) {
		let char = if permission == Permission::Yes { permission_and_special_char } else { special_only_char };
		print_char(buf, x, y, char, color);
	} else {
		print_permission(buf, x, y, permission, permission_only_char, color);
	}
}

fn print_char(buf: &mut Buffer, x: u16, y: u16, char: char, color: Color) {
	buf.get_mut(x, y).set_char(char).set_fg(color);
}
