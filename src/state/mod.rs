use std::path::Path;

use slab_tree::{NodeId, NodeRef};

use crate::state::filesystem::{FsTree, FsTreeViewNode};

pub mod action;
pub mod filesystem;

pub struct State {
	pub tree: FsTree,
	pub selected_view_node_id: NodeId,
}

impl State {
	pub fn with_root_path(root_path: &Path) -> Self {
		let tree = FsTree::with_root_path(root_path);
		let selected_view_node_id = tree.view.root_id();
		
		Self { tree, selected_view_node_id }
	}
	
	pub fn selected_node(&self) -> Option<NodeRef<FsTreeViewNode>> {
		return self.tree.view.get(self.selected_view_node_id);
	}
}
