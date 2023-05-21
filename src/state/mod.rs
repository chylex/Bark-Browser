use std::path::Path;

use slab_tree::{NodeId, NodeRef};

pub use self::tree::FileSystemTree;
pub use self::tree::Node;

mod tree;
pub mod action;

pub struct State {
	pub tree: FileSystemTree,
	pub selected_id: NodeId,
}

impl State {
	pub fn with_root_path(root_path: &Path) -> Self {
		let tree = FileSystemTree::with_root_path(root_path);
		let selected_id = tree.root_id;
		
		Self { tree, selected_id }
	}
	
	pub fn get_selected_node(&self) -> Option<NodeRef<Node>> {
		return self.tree.get(self.selected_id);
	}
}
