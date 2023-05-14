use std::ffi::OsStr;
use std::io;
use std::io::{stdout, Stdout, Write};

use crossterm::{Command, QueueableCommand};
use crossterm::cursor::{MoveTo, MoveToNextLine};
use crossterm::style::{Color, ContentStyle, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize};
use crossterm::terminal::{Clear, ClearType};

use crate::file::{FileEntry, FileKind, Permission};
use crate::state::State;

type R = io::Result<()>;

pub struct View {
	out: Stdout,
}

impl View {
	pub fn stdout() -> Self {
		Self { out: stdout() }
	}
	
	fn queue(&mut self, command: impl Command) -> R {
		self.out.queue(command)?;
		Ok(())
	}
	
	pub fn render_state(&mut self, state: &State) -> R {
		self.queue(Clear(ClearType::All))?;
		self.queue(MoveTo(0, 0))?;
		
		if let Some(root) = state.tree.get(state.tree.root_id) {
			for node in root.traverse_pre_order() {
				let level = node.ancestors().count();
				self.print_node(level, &node.data().entry, node.node_id() == state.selected_id)?;
			}
		}
		
		self.out.flush()?;
		Ok(())
	}
	
	fn print_node(&mut self, level: usize, entry: &FileEntry, is_selected: bool) -> R {
		let name = entry.name().and_then(OsStr::to_str).unwrap_or("???");
		let mode = entry.mode();
		
		self.queue(ResetColor)?;
		self.queue(Print(" ".repeat(level)))?;
		
		if is_selected {
			self.queue(SetForegroundColor(Color::Black))?;
			self.queue(SetBackgroundColor(Color::White))?;
		} else {
			self.queue(SetForegroundColor(Color::White))?;
			self.queue(SetBackgroundColor(Color::Black))?;
		}
		
		self.queue(Print(name))?;
		self.queue(ResetColor)?;
		self.queue(Print(" "))?;
		
		self.print_kind(entry.kind())?;
		
		let user = mode.user();
		let group = mode.group();
		let others = mode.others();
		
		self.print_permission(user.read(), 'r', Color::DarkBlue)?;
		self.print_permission(user.write(), 'w', Color::DarkRed)?;
		self.print_permission_or_special(user.execute(), mode.is_setuid(), 'x', 'S', 's', Color::DarkGreen)?;
		
		self.print_permission(group.read(), 'r', Color::DarkBlue)?;
		self.print_permission(group.write(), 'w', Color::DarkRed)?;
		self.print_permission_or_special(group.execute(), mode.is_setgid(), 'x', 'S', 's', Color::DarkGreen)?;
		
		self.print_permission(others.read(), 'r', Color::DarkBlue)?;
		self.print_permission(others.write(), 'w', Color::DarkRed)?;
		self.print_permission_or_special(others.execute(), mode.is_sticky(), 'x', 'T', 't', Color::DarkGreen)?;
		
		self.queue(MoveToNextLine(1))?;
		
		Ok(())
	}
	
	fn print_kind(&mut self, kind: FileKind) -> R {
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
		
		self.print_char(c, Color::Grey)
	}
	
	fn print_permission(&mut self, permission: Permission, c: char, color: Color) -> R {
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
		
		self.print_char(c, color)
	}
	
	fn print_permission_or_special(&mut self, permission: Permission, special: Option<bool>, permission_only_char: char, special_only_char: char, permission_and_special_char: char, color: Color) -> R {
		if matches!(special, Some(true)) {
			let char = if permission == Permission::Yes { permission_and_special_char } else { special_only_char };
			self.print_char(char, color)
		} else {
			self.print_permission(permission, permission_only_char, color)
		}
	}
	
	fn print_char(&mut self, char: char, color: Color) -> R {
		self.queue(Print(ContentStyle::new().with(color).apply(char)))
	}
}
