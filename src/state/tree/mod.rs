use std::fs::ReadDir;
use std::path::Path;

use slab_tree::{NodeId, NodeMut, NodeRef, Tree};

use crate::file::FileEntry;

mod above_below;
mod expand_collapse;
mod root_node;

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

fn get_directory_children(entry: &FileEntry) -> Option<Vec<FileEntry>> {
	entry.path()
	     .and_then(|path| std::fs::read_dir(path).ok())
	     .map(read_directory_children)
}

fn read_directory_children(reader: ReadDir) -> Vec<FileEntry> {
	let mut children = reader
		.map(|e| e.as_ref().map(FileEntry::from).unwrap_or_else(|_| FileEntry::dummy()))
		.collect::<Vec<_>>();
	
	children.sort_by(|f1, f2| f1.name().str().cmp(f2.name().str()));
	children
}
