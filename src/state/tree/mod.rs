use std::fs::ReadDir;
use std::path::Path;

use slab_tree::{NodeId, NodeMut, NodeRef, Tree};
use slab_tree::iter::PreOrder;

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

impl<'a> IntoIterator for &'a FileSystemTree {
	type Item = NodeRef<'a, Node>;
	type IntoIter = FileSystemTreeIterator<'a>;
	
	fn into_iter(self) -> Self::IntoIter {
		FileSystemTreeIterator {
			iter: self.inner.root().map(|root| root.traverse_pre_order())
		}
	}
}

pub struct FileSystemTreeIterator<'a> {
	iter: Option<PreOrder<'a, Node>>
}

impl<'a> Iterator for FileSystemTreeIterator<'a> {
	type Item = NodeRef<'a, Node>;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.as_mut().and_then(PreOrder::next)
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
