use std::cmp::min;
use std::io;
use std::io::{stdout, Stdout, Write};

use crossterm::{Command, QueueableCommand, terminal};
use crossterm::cursor::MoveTo;
use crossterm::style::{Print, ResetColor};
use crossterm::terminal::{Clear, ClearType};
use slab_tree::{NodeId, NodeRef};

use crate::file::FileEntry;
use crate::gui::components;
use crate::state::{FileSystemTree, Node, State};

pub type R = io::Result<()>;

pub struct View {
	out: Stdout,
}

impl View {
	pub fn stdout() -> Self {
		Self { out: stdout() }
	}
	
	pub fn queue(&mut self, command: impl Command) -> R {
		self.out.queue(command)?;
		Ok(())
	}
	
	pub fn flush(&mut self) -> R {
		self.out.flush()
	}
	
	pub fn render_state(&mut self, state: &State) -> R {
		let terminal_size = terminal::size()?;
		
		SingleFrame {
			view: self,
			cols: terminal_size.0 as usize,
			rows: terminal_size.1 as usize,
			state,
		}.render()
	}
}

struct SingleFrame<'a> {
	view: &'a mut View,
	cols: usize,
	rows: usize,
	state: &'a State,
}

impl<'a> SingleFrame<'a> {
	fn render(&mut self) -> R {
		if let Some(middle_node) = self.state.get_selected_node().or_else(|| self.state.tree.get(self.state.tree.root_id)) {
			self.render_tree(middle_node)?;
			self.view.flush()?;
		}
		
		Ok(())
	}
	
	fn render_tree(&mut self, middle_node: NodeRef<Node>) -> R {
		let displayed_rows = self.collect_displayed_rows(self.rows, middle_node);
		
		let max_name_column_width = self.cols.saturating_sub(components::date_time::COLUMN_WIDTH + 2 + components::file_permissions::COLUMN_WIDTH + 2);
		let name_column_width = min(max_name_column_width, displayed_rows.iter().map(|row| row.level + row.entry.name().len()).max().unwrap_or(0));
		
		for (index, row) in displayed_rows.iter().enumerate() {
			if let Ok(row_index) = u16::try_from(index) {
				self.render_node(row_index, row.level, row.entry, name_column_width, row.node_id == self.state.selected_id)?;
			} else {
				break;
			}
		};
		
		for y in displayed_rows.len()..self.rows {
			if let Ok(row_index) = u16::try_from(y) {
				self.view.queue(MoveTo(0, row_index))?;
				self.view.queue(Clear(ClearType::UntilNewLine))?;
			} else {
				break;
			}
		}
		
		Ok(())
	}
	
	fn collect_displayed_rows(&self, terminal_rows: usize, middle_node: NodeRef<'a, Node>) -> Vec<NodeRow<'a>> {
		let mut displayed_rows = Vec::with_capacity(terminal_rows);
		displayed_rows.push(NodeRow::from(&middle_node));
		
		let mut cursor_up_id = Some(middle_node.node_id());
		let mut cursor_down_id = Some(middle_node.node_id());
		
		while displayed_rows.len() < terminal_rows {
			if let Some(next_node_up) = self.move_cursor(&mut cursor_up_id, FileSystemTree::get_node_above) {
				displayed_rows.insert(0, NodeRow::from(&next_node_up));
			}
			
			if displayed_rows.len() >= terminal_rows {
				break;
			}
			
			if let Some(next_node_down) = self.move_cursor(&mut cursor_down_id, FileSystemTree::get_node_below) {
				displayed_rows.push(NodeRow::from(&next_node_down));
			}
			
			if cursor_up_id.is_none() && cursor_down_id.is_none() {
				break;
			}
		}
		
		displayed_rows
	}
	
	fn move_cursor<F>(&self, cursor: &mut Option<NodeId>, func: F) -> Option<NodeRef<'a, Node>> where F: FnOnce(&FileSystemTree, &NodeRef<Node>) -> Option<NodeId> {
		let tree = &self.state.tree;
		let next_node = cursor
			.and_then(|id| tree.get(id))
			.and_then(|node| func(tree, &node))
			.and_then(|id| tree.get(id));
		
		*cursor = next_node.as_ref().map(NodeRef::node_id);
		
		next_node
	}
	
	fn render_node(&mut self, row: u16, level: usize, entry: &FileEntry, name_column_width: usize, is_selected: bool) -> R {
		self.view.queue(MoveTo(0, row))?;
		
		components::file_name::print(self.view, entry.name(), level, name_column_width, is_selected)?;
		
		self.print_column_separator()?;
		
		components::date_time::print(self.view, entry.modified_time())?;
		
		self.print_column_separator()?;
		
		components::file_permissions::print(self.view, entry.kind(), entry.mode())?;
		
		self.view.queue(ResetColor)?;
		self.view.queue(Clear(ClearType::UntilNewLine))?;
		Ok(())
	}
	
	fn print_column_separator(&mut self) -> R {
		self.view.queue(ResetColor)?;
		self.view.queue(Print("  "))
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
