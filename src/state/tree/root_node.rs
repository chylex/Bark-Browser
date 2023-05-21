use std::path::Path;

use slab_tree::{NodeId, NodeMut};

use crate::file::FileEntry;
use crate::state::{FileSystemTree, Node};
use crate::state::tree::get_directory_children;

impl FileSystemTree {
	pub fn traverse_up_root(&mut self) -> Option<NodeId> {
		if let Some(new_root) = self.get_root_parent() {
			self.root_id = self.inner.set_root(new_root);
			
			if let Some(mut new_root) = self.inner.root_mut() {
				expand_new_root(&mut new_root);
			}
			
			Some(self.root_id)
		} else {
			None
		}
	}
	
	fn get_root_parent(&mut self) -> Option<Node> {
		self.inner.get(self.root_id)
		    .and_then(|root| root.data().entry.path())
		    .and_then(Path::parent)
		    .map(FileEntry::from)
		    .map(Node::from)
	}
}

fn expand_new_root(new_root: &mut NodeMut<Node>) {
	if let Some(children) = get_directory_children(&new_root.data().entry) {
		let old_root_path = new_root.first_child().and_then(|mut node| node.data().entry.path().map(Path::to_path_buf));
		let old_root_path = old_root_path.as_deref();
		
		for child in children {
			if child.path() != old_root_path {
				new_root.append(Node::from(child));
			} else if let Some(mut old_root) = new_root.first_child() {
				old_root.make_last_sibling();
			}
		}
	}
}
