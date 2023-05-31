use std::path::Path;

use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::tree::{FsTree, FsTreeViewNode};
use crate::file::FileOwnerNameCache;
use crate::state::action::{ActionResult, KeyBinding};
use crate::state::layer::Layer;
use crate::state::view::{R, View};

mod action;
mod render;
mod tree;

pub struct FsLayer {
	pub tree: FsTree,
	pub selected_view_node_id: NodeId,
	file_owner_name_cache: FileOwnerNameCache,
	column_width_cache: Option<ColumnWidths>,
}

impl FsLayer {
	pub fn with_root_path(root_path: &Path) -> Self {
		let mut tree = FsTree::with_root_path(root_path);
		let root_id = tree.view.root_id();
		
		tree.expand(root_id);
		
		Self {
			tree,
			selected_view_node_id: root_id,
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
		action::ACTION_MAP.handle_action(self, key_binding)
	}
	
	fn render(&mut self, view: &mut View) -> R {
		render::render(view, self)
	}
}

#[derive(Copy, Clone, Default)]
struct ColumnWidths {
	name: usize,
	user: usize,
	group: usize,
}
