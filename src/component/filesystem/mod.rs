use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::event::FsLayerEvent;
use crate::component::filesystem::tree::{FsTree, FsTreeViewNode};
use crate::file::FileOwnerNameCache;
use crate::input::keymap::{KeyBinding, KeyMapLookupResult};
use crate::state::action::ActionResult;
use crate::state::layer::Layer;
use crate::state::view::F;

mod action;
mod event;
mod render;
mod tree;

pub struct FsLayer {
	pub tree: FsTree,
	pub selected_view_node_id: NodeId,
	pending_keys: Vec<KeyBinding>,
	pending_events: Rc<RefCell<Vec<FsLayerEvent>>>,
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
			pending_keys: Vec::new(),
			pending_events: Rc::new(RefCell::new(Vec::new())),
			file_owner_name_cache: FileOwnerNameCache::new(),
			column_width_cache: None,
		}
	}
	
	pub fn tree_structure_changed(&mut self) {
		self.column_width_cache.take();
	}
	
	pub fn selected_node(&self) -> Option<NodeRef<FsTreeViewNode>> {
		return self.tree.view.get(self.selected_view_node_id);
	}
}

impl Layer for FsLayer {
	fn handle_input(&mut self, key_binding: KeyBinding) -> ActionResult {
		self.pending_keys.push(key_binding);
		
		match action::ACTION_MAP.lookup(&self.pending_keys) {
			KeyMapLookupResult::Prefix => {
				ActionResult::Nothing
			}
			
			KeyMapLookupResult::Found(action) => {
				self.pending_keys.clear();
				action.perform(self)
			}
			
			KeyMapLookupResult::None => {
				self.pending_keys.clear();
				ActionResult::Nothing
			}
		}
	}
	
	fn render(&mut self, frame: &mut F) {
		for event in self.pending_events.take() {
			event.handle(self);
		}
		
		render::render(self, frame)
	}
}

#[derive(Copy, Clone, Default)]
struct ColumnWidths {
	name: u16,
	user: u16,
	group: u16,
}
