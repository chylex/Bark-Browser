//! Components should provide a function for either rendering or printing themselves.
//!
//! Both must set their desired foreground and background colors at the beginning, and should not restore them to the original values.
//! 
//! Rendering functions may affect any part of the terminal, and may leave the cursor in any position.
//! 
//! Printing functions must only affect the rest of the current line or lines below the cursor's initial position, and must leave the cursor
//! after the end of printed content that is intended to remain. Anything past the final cursor position will be overwritten.

use std::cmp::{max, min};

use crossterm::cursor::MoveTo;
use crossterm::style::{Print, ResetColor};
use crossterm::terminal;
use crossterm::terminal::{Clear, ClearType};
use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::{ColumnWidths, FsLayer};
use crate::component::filesystem::tree::{FsTree, FsTreeView, FsTreeViewNode};
use crate::file::{FileEntry, FileKind, FileOwnerNameCache};
use crate::state::view::{R, View};

mod column;
mod date_time;
mod file_name;
mod file_owner;
mod file_permissions;
mod file_size;

pub fn render(view: &mut View, layer: &mut FsLayer) -> R {
	let terminal_size = terminal::size()?;
	let cols = terminal_size.0 as usize;
	let rows = terminal_size.1 as usize;
	
	let column_widths = get_or_update_column_widths(layer, cols);
	let displayed_rows = collect_displayed_rows(&layer.tree, layer.selected_view_node_id, rows);
	let file_owner_name_cache = &mut layer.file_owner_name_cache;
	
	SingleFrame { view, rows, column_widths, file_owner_name_cache }.render(displayed_rows)
}

fn get_or_update_column_widths(layer: &mut FsLayer, cols: usize) -> ColumnWidths {
	let mut column_widths = *layer.column_width_cache.get_or_insert_with(|| {
		let mut result = ColumnWidths::default();
		
		for node in layer.tree.view.into_iter() {
			let entry = layer.tree.get_entry(&node).unwrap_or_else(|| FileEntry::dummy_as_ref());
			
			result.name = max(result.name, get_node_level(&node) + entry.name().len());
			result.user = max(result.user, layer.file_owner_name_cache.get_user(entry.uid()).len());
			result.group = max(result.group, layer.file_owner_name_cache.get_group(entry.gid()).len());
		}
		
		result
	});
	
	let owner_column_width = column_widths.user + 1 + column_widths.group;
	let max_name_width = cols.saturating_sub(2 + file_size::COLUMN_WIDTH + 2 + date_time::COLUMN_WIDTH + 2 + owner_column_width + 2 + file_permissions::COLUMN_WIDTH);
	
	column_widths.name = min(column_widths.name, max_name_width);
	column_widths
}

fn collect_displayed_rows(tree: &FsTree, selected_node_id: NodeId, terminal_rows: usize) -> Vec<NodeRow> {
	let mut displayed_rows = Vec::with_capacity(terminal_rows);
	
	if let Some(middle_node) = tree.view.get(selected_node_id).or_else(|| tree.view.root()) {
		let middle_node_id = middle_node.node_id();
		
		displayed_rows.push(NodeRow::from(&middle_node, tree, middle_node_id == selected_node_id));
		
		let mut cursor_up_id = Some(middle_node_id);
		let mut cursor_down_id = Some(middle_node_id);
		
		while displayed_rows.len() < terminal_rows {
			if let Some(next_node_up) = move_cursor(tree, &mut cursor_up_id, FsTreeView::get_node_above) {
				displayed_rows.insert(0, NodeRow::from(&next_node_up, tree, false));
			}
			
			if displayed_rows.len() >= terminal_rows {
				break;
			}
			
			if let Some(next_node_down) = move_cursor(tree, &mut cursor_down_id, FsTreeView::get_node_below) {
				displayed_rows.push(NodeRow::from(&next_node_down, tree, false));
			}
			
			if cursor_up_id.is_none() && cursor_down_id.is_none() {
				break;
			}
		}
	}
	
	displayed_rows
}

fn move_cursor<'a, F>(tree: &'a FsTree, cursor: &mut Option<NodeId>, func: F) -> Option<NodeRef<'a, FsTreeViewNode>> where F: FnOnce(&FsTreeView, &NodeRef<FsTreeViewNode>) -> Option<NodeId> {
	let view = &tree.view;
	let next_node = cursor
		.and_then(|id| view.get(id))
		.and_then(|node| func(view, &node))
		.and_then(|id| view.get(id));
	
	*cursor = next_node.as_ref().map(NodeRef::node_id);
	
	next_node
}

struct SingleFrame<'a> {
	view: &'a mut View,
	rows: usize,
	column_widths: ColumnWidths,
	file_owner_name_cache: &'a mut FileOwnerNameCache,
}

impl<'a> SingleFrame<'a> {
	fn render(&mut self, rows: Vec<NodeRow<'a>>) -> R {
		for (index, row) in rows.iter().enumerate() {
			if let Ok(row_index) = u16::try_from(index) {
				self.render_row(row_index, row)?;
			} else {
				break;
			}
		};
		
		for y in rows.len()..self.rows {
			if let Ok(row_index) = u16::try_from(y) {
				self.view.queue(MoveTo(0, row_index))?;
				self.view.queue(Clear(ClearType::UntilNewLine))?;
			} else {
				break;
			}
		}
		
		Ok(())
	}
	
	fn render_row(&mut self, row_index: u16, row: &NodeRow) -> R {
		let entry = row.entry;
		
		self.view.queue(MoveTo(0, row_index))?;
		
		file_name::print(self.view, entry, row.level, self.column_widths.name, row.is_selected)?;
		
		self.print_column_separator()?;
		
		file_size::print(self.view, if let FileKind::File { size } = entry.kind() { Some(*size) } else { None })?;
		
		self.print_column_separator()?;
		
		date_time::print(self.view, entry.modified_time())?;
		
		self.print_column_separator()?;
		
		file_owner::print(self.view, self.file_owner_name_cache.get_user(entry.uid()), self.column_widths.user)?;
		self.view.queue(Print(" "))?;
		file_owner::print(self.view, self.file_owner_name_cache.get_group(entry.gid()), self.column_widths.group)?;
		
		self.print_column_separator()?;
		
		file_permissions::print(self.view, entry.kind(), entry.mode())?;
		
		self.view.queue(ResetColor)?;
		self.view.queue(Clear(ClearType::UntilNewLine))
	}
	
	fn print_column_separator(&mut self) -> R {
		self.view.queue(ResetColor)?;
		self.view.queue(Print("  "))
	}
}

struct NodeRow<'a> {
	level: usize,
	entry: &'a FileEntry,
	is_selected: bool,
}

impl<'a> NodeRow<'a> {
	fn from(view_node: &NodeRef<'a, FsTreeViewNode>, tree: &'a FsTree, is_selected: bool) -> Self {
		return Self {
			level: get_node_level(view_node),
			entry: tree.get_entry(view_node).unwrap_or_else(|| FileEntry::dummy_as_ref()),
			is_selected,
		};
	}
}

fn get_node_level<T>(node: &NodeRef<T>) -> usize {
	node.ancestors().count()
}
