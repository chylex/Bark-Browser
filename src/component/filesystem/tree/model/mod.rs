use std::path::Path;

use slab_tree::{NodeId, NodeMut, NodeRef, RemoveBehavior, Tree};

use crate::file::FileEntry;

mod children;
mod parents;
mod refresh;

pub struct FsTreeModel {
	inner: Tree<FsTreeModelNode>,
	root_id: NodeId,
}

impl FsTreeModel {
	pub fn with_root_path(path: &Path) -> Self {
		let mut inner = Tree::new();
		let root_id = inner.set_root(FsTreeModelNode::from(FileEntry::from(path)));
		
		Self { inner, root_id }
	}
	
	pub fn root_id(&self) -> NodeId {
		self.root_id
	}
	
	pub fn get(&self, node_id: NodeId) -> Option<NodeRef<FsTreeModelNode>> {
		self.inner.get(node_id)
	}
	
	pub fn get_mut(&mut self, node_id: NodeId) -> Option<NodeMut<FsTreeModelNode>> {
		self.inner.get_mut(node_id)
	}
	
	pub fn remove(&mut self, node_id: NodeId) -> Option<FsTreeModelNode> {
		self.inner.remove(node_id, RemoveBehavior::DropChildren)
	}
}

pub struct FsTreeModelNode {
	pub entry: FileEntry,
	are_children_known: bool,
}

impl From<FileEntry> for FsTreeModelNode {
	fn from(entry: FileEntry) -> Self {
		Self { entry, are_children_known: false }
	}
}
