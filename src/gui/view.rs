use std::ffi::OsStr;
use std::io;
use std::io::{stdout, Stdout, Write};

use crossterm::{Command, QueueableCommand, terminal};
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, ContentStyle, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize};
use crossterm::terminal::{Clear, ClearType};
use slab_tree::{NodeId, NodeRef};

use crate::file::{FileEntry, FileKind, Permission};
use crate::state::{FileSystemTree, Node, State};

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
		let terminal_rows = terminal::size()?.1 as usize;
		
		if let Some(middle_node) = state.tree.get(state.selected_id).or_else(|| state.tree.get(state.tree.root_id)) {
			self.render_tree(state, terminal_rows, middle_node)?;
			self.out.flush()?;
		}
		
		Ok(())
	}
	
	fn render_tree(&mut self, state: &State, terminal_rows: usize, middle_node: NodeRef<Node>) -> R {
		let displayed_rows = collect_displayed_rows(state, terminal_rows, middle_node);
		
		for (index, row) in displayed_rows.iter().enumerate() {
			if let Ok(row_index) = u16::try_from(index) {
				self.render_node(row_index, row.level, row.entry, row.node_id == state.selected_id)?;
			} else {
				break;
			}
		};
		
		for y in displayed_rows.len()..terminal_rows {
			if let Ok(row_index) = u16::try_from(y) {
				self.queue(MoveTo(0, row_index))?;
				self.queue(Clear(ClearType::UntilNewLine))?;
			} else {
				break;
			}
		}
		
		Ok(())
	}
	
	fn render_node(&mut self, row: u16, level: usize, entry: &FileEntry, is_selected: bool) -> R {
		let name = entry.name().and_then(OsStr::to_str).unwrap_or("???");
		let mode = entry.mode();
		
		self.queue(MoveTo(0, row))?;
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
		
		self.queue(Clear(ClearType::UntilNewLine))?;
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

struct NodeRow<'a> {
	level: usize,
	node_id: NodeId,
	entry: &'a FileEntry,
}

impl<'a> From<&NodeRef<'a, Node>> for NodeRow<'a> {
	fn from(node: &NodeRef<'a, Node>) -> Self {
		return Self {
			level: node.ancestors().count(),
			node_id: node.node_id(),
			entry: &node.data().entry,
		};
	}
}

fn collect_displayed_rows<'a>(state: &'a State, terminal_rows: usize, middle_node: NodeRef<'a, Node>) -> Vec<NodeRow<'a>> {
	let mut displayed_rows = Vec::with_capacity(terminal_rows);
	displayed_rows.push(NodeRow::from(&middle_node));
	
	let mut cursor_up_id = Some(middle_node.node_id());
	let mut cursor_down_id = Some(middle_node.node_id());
	
	while displayed_rows.len() < terminal_rows {
		if let Some(next_node_up) = move_cursor(&mut cursor_up_id, state, FileSystemTree::get_node_above) {
			displayed_rows.insert(0, NodeRow::from(&next_node_up));
		}
		
		if displayed_rows.len() >= terminal_rows {
			break;
		}
		
		if let Some(next_node_down) = move_cursor(&mut cursor_down_id, state, FileSystemTree::get_node_below) {
			displayed_rows.push(NodeRow::from(&next_node_down));
		}
		
		if cursor_up_id.is_none() && cursor_down_id.is_none() {
			break;
		}
	}
	
	displayed_rows
}

fn move_cursor<'a, F>(cursor: &mut Option<NodeId>, state: &'a State, func: F) -> Option<NodeRef<'a, Node>> where F: FnOnce(&FileSystemTree, &NodeRef<Node>) -> Option<NodeId> {
	let next_node = cursor
		.and_then(|id| state.tree.get(id))
		.and_then(|node| func(&state.tree, &node))
		.and_then(|id| state.tree.get(id));
	
	*cursor = next_node.as_ref().map(NodeRef::node_id);
	
	next_node
}
