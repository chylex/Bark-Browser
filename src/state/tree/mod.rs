use std::path::Path;

use slab_tree::{NodeId, NodeMut, NodeRef, Tree};

use crate::file::FileEntry;

mod above_below;
mod expand_collapse;

pub struct FileSystemTree {
	inner: Tree<Node>,
	pub root_id: NodeId,
}

impl FileSystemTree {
	pub fn with_root_path(path: &Path) -> Self {
		let mut inner = Tree::new();
		let root_id = inner.set_root(Node::from(FileEntry::from(path)));
		
		Self { inner, root_id }
	}
	
	pub fn get(&self, node_id: NodeId) -> Option<NodeRef<Node>> {
		self.inner.get(node_id)
	}
	
	pub fn get_mut(&mut self, node_id: NodeId) -> Option<NodeMut<Node>> {
		self.inner.get_mut(node_id)
	}
}

pub struct Node {
	pub entry: FileEntry,
	is_expanded: bool,
}

impl From<FileEntry> for Node {
	fn from(entry: FileEntry) -> Self {
		Self { entry, is_expanded: false }
	}
}
