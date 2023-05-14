use std::path::Path;

use slab_tree::{NodeId, NodeMut, NodeRef, Tree};

use crate::file::FileEntry;

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
	
	pub fn expand(&mut self, node_id: NodeId) -> bool {
		if let Some(mut node) = self.get_mut(node_id) {
			let data = node.data();
			if data.is_expanded {
				return false;
			}
			
			data.is_expanded = true;
			
			if let Some(child_entries) = data.entry.path().and_then(|path| std::fs::read_dir(path).ok()) {
				let mut children = child_entries
					.map(|e| e.as_ref().map(FileEntry::from).unwrap_or_else(|_| FileEntry::dummy()))
					.collect::<Vec<_>>();
				
				children.sort_by(|f1, f2| f1.name().cmp(&f2.name()));
				
				for child in children {
					node.append(Node::from(child));
				}
				
				return true;
			}
		}
		false
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
