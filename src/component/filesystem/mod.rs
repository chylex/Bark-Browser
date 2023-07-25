use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::event::FsLayerPendingEvents;
use crate::component::filesystem::registers::FsTreeRegisters;
use crate::component::filesystem::tree::{FsTree, FsTreeViewNode};
use crate::file::FileOwnerNameCache;
use crate::input::keymap::{KeyBinding, KeyMapLookupResult};
use crate::state::action::ActionResult;
use crate::state::Environment;
use crate::state::layer::Layer;
use crate::state::view::F;

mod action;
mod event;
mod render;
mod tree;
mod registers;

pub struct FsLayer {
	pub tree: FsTree,
	pub selected_view_node_id: NodeId,
	pub registers: FsTreeRegisters,
	cursor_y: u16,
	pending_keys: Vec<KeyBinding>,
	pending_events: FsLayerPendingEvents,
	file_owner_name_cache: FileOwnerNameCache,
	column_width_cache: Option<ColumnWidths>,
}

impl FsLayer {
	pub fn with_root_path(root_path: &Path) -> Self {
		// Initialize action map early in case it errors.
		let _ = *action::ACTION_MAP;
		
		let mut tree = FsTree::with_root_path(root_path);
		let root_id = tree.view.root_id();
		
		tree.expand(root_id);
		
		Self {
			tree,
			selected_view_node_id: root_id,
			cursor_y: 0,
			registers: FsTreeRegisters::new(),
			pending_keys: Vec::new(),
			pending_events: Rc::new(RefCell::new(Vec::new())),
			file_owner_name_cache: FileOwnerNameCache::new(),
			column_width_cache: None,
		}
	}
	
	pub const fn dialog_y(&self) -> u16 {
		self.cursor_y.saturating_add(1)
	}
	
	pub fn selected_node(&self) -> Option<NodeRef<FsTreeViewNode>> {
		return self.tree.view.get(self.selected_view_node_id);
	}
	
	pub fn expand(&mut self, view_node_id: NodeId) -> bool {
		let result = self.tree.expand(view_node_id);
		tree_structure_changed_if_true(self, result)
	}
	
	pub fn collapse(&mut self, view_node_id: NodeId) -> bool {
		let result = self.tree.collapse(view_node_id);
		tree_structure_changed_if_true(self, result)
	}
	
	pub fn expand_or_collapse(&mut self, view_node_id: NodeId) -> bool {
		let result = self.tree.expand_or_collapse(view_node_id);
		tree_structure_changed_if_true(self, result)
	}
	
	pub fn refresh_children(&mut self, view_node_id: NodeId) -> bool {
		let result = self.tree.refresh_children(view_node_id);
		tree_structure_changed_if_true(self, result)
	}
	
	pub fn traverse_up_root(&mut self) -> Option<NodeId> {
		let new_root_id = self.tree.traverse_up_root();
		tree_structure_changed_if_true(self, new_root_id.is_some());
		new_root_id
	}
}

fn tree_structure_changed(layer: &mut FsLayer) {
	layer.column_width_cache.take();
}

fn tree_structure_changed_if_true(layer: &mut FsLayer, result: bool) -> bool {
	if result {
		tree_structure_changed(layer);
	}
	result
}

impl Layer for FsLayer {
	fn handle_input(&mut self, environment: &Environment, key_binding: KeyBinding) -> ActionResult {
		self.pending_keys.push(key_binding);
		
		match action::ACTION_MAP.lookup(&self.pending_keys) {
			KeyMapLookupResult::Prefix => {
				ActionResult::Nothing
			}
			
			KeyMapLookupResult::Found(action) => {
				self.pending_keys.clear();
				
				let old_count = self.registers.count;
				let result = action.perform(self, environment);
				
				// Reset count after every action, unless the action modified it.
				if old_count == self.registers.count {
					self.registers.count = None;
				}
				
				result
			}
			
			KeyMapLookupResult::None => {
				self.pending_keys.clear();
				self.registers.count = None;
				ActionResult::Nothing
			}
		}
	}
	
	fn render(&mut self, frame: &mut F) {
		for event in self.pending_events.take() {
			event.handle(self);
		}
		
		render::render(self, frame);
	}
}

#[derive(Copy, Clone, Default)]
struct ColumnWidths {
	name: u16,
	user: u16,
	group: u16,
}
