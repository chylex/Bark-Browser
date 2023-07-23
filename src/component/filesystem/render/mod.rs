use std::cmp::{max, min};

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Span;
use ratatui::widgets::{Clear, Widget};
use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::{ColumnWidths, FsLayer};
use crate::component::filesystem::tree::{FsTree, FsTreeView, FsTreeViewNode};
use crate::file::{FileEntry, FileKind, FileOwnerNameCache};
use crate::state::view::F;

mod column;
mod date_time;
mod file_name;
mod file_owner;
mod file_permissions;
mod file_size;

pub fn render(layer: &mut FsLayer, frame: &mut F) {
	let size = frame.size();
	
	let column_widths = get_or_update_column_widths(layer, size.width);
	let file_owner_name_cache = &mut layer.file_owner_name_cache;
	
	let rows = collect_displayed_rows(&layer.tree, layer.selected_view_node_id, size.height as usize);
	
	frame.render_widget(Clear, size);
	frame.render_widget(FsWidget { rows, column_widths, file_owner_name_cache }, size);
}

fn get_or_update_column_widths(layer: &mut FsLayer, cols: u16) -> ColumnWidths {
	let mut column_widths = *layer.column_width_cache.get_or_insert_with(|| {
		let mut name: usize = 0;
		let mut user: usize = 0;
		let mut group: usize = 0;
		
		for node in &layer.tree.view {
			let entry = layer.tree.get_entry(&node).unwrap_or_else(|| FileEntry::dummy_as_ref());
			
			name = max(name, get_node_level(&node).saturating_add(Span::from(entry.name().str()).width()));
			user = max(user, layer.file_owner_name_cache.get_user(entry.uid()).len());
			group = max(group, layer.file_owner_name_cache.get_group(entry.gid()).len());
		}
		
		ColumnWidths {
			name: u16::try_from(name).unwrap_or(u16::MAX),
			user: u16::try_from(user).unwrap_or(u16::MAX),
			group: u16::try_from(group).unwrap_or(u16::MAX),
		}
	});
	
	let owner_column_width_padded = column_widths.user.saturating_add(1).saturating_add(column_widths.group).saturating_add(2);
	let max_name_column_width = cols.saturating_sub(2 + file_size::COLUMN_WIDTH + 2 + date_time::COLUMN_WIDTH + 2 + file_permissions::COLUMN_WIDTH).saturating_sub(owner_column_width_padded);
	
	column_widths.name = min(column_widths.name, max_name_column_width);
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

struct FsWidget<'a> {
	rows: Vec<NodeRow<'a>>,
	column_widths: ColumnWidths,
	file_owner_name_cache: &'a mut FileOwnerNameCache,
}

impl Widget for FsWidget<'_> {
	fn render(self, _area: Rect, buf: &mut Buffer) {
		for (index, row) in self.rows.iter().enumerate() {
			if let Ok(row_index) = u16::try_from(index) {
				row.render(buf, row_index, &self.column_widths, self.file_owner_name_cache);
			} else {
				break;
			}
		};
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
	
	#[allow(clippy::trivially_copy_pass_by_ref)]
	fn render(&self, buf: &mut Buffer, y: u16, column_widths: &ColumnWidths, file_owner_name_cache: &mut FileOwnerNameCache) {
		let entry = self.entry;
		
		let width = buf.area().width;
		let mut x = 0;
		
		file_name::print(buf, x, y, entry, self.level, column_widths.name, self.is_selected);
		x = x.saturating_add(column_widths.name).saturating_add(2);
		
		if exceeds_width(x, file_size::COLUMN_WIDTH, width) {
			return;
		}
		
		file_size::print(buf, x, y, if let FileKind::File { size } = entry.kind() { Some(*size) } else { None });
		x = x.saturating_add(file_size::COLUMN_WIDTH).saturating_add(2);
		
		if exceeds_width(x, date_time::COLUMN_WIDTH, width) {
			return;
		}
		
		date_time::print(buf, x, y, entry.modified_time());
		x = x.saturating_add(date_time::COLUMN_WIDTH).saturating_add(2);
		
		if exceeds_width(x, column_widths.user.saturating_add(1).saturating_add(column_widths.group), width) {
			return;
		}
		
		file_owner::print(buf, x, y, file_owner_name_cache.get_user(entry.uid()), column_widths.user);
		x = x.saturating_add(column_widths.user).saturating_add(1);
		
		file_owner::print(buf, x, y, file_owner_name_cache.get_group(entry.gid()), column_widths.group);
		x = x.saturating_add(column_widths.group).saturating_add(2);
		
		if exceeds_width(x, file_permissions::COLUMN_WIDTH, width) {
			return;
		}
		
		file_permissions::print(buf, x, y, entry.kind(), entry.mode());
	}
}

fn exceeds_width(x: u16, column_width: u16, terminal_width: u16) -> bool {
	let x = x.checked_add(column_width);
	x.is_none() || x.is_some_and(|x| x > terminal_width)
}

fn get_node_level<T>(node: &NodeRef<T>) -> usize {
	node.ancestors().count()
}
