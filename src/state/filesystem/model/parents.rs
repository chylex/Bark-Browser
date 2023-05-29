use std::path::Path;

use slab_tree::{NodeId, NodeMut};

use crate::file::FileEntry;
use crate::state::filesystem::{FsTreeModel, FsTreeModelNode};

impl FsTreeModel {
	pub fn traverse_up_root(&mut self) -> Option<NodeId> {
		let old_root_path = self.inner.root().and_then(|node| node.data().entry.path().map(Path::to_path_buf));
		let old_root_path = old_root_path.as_deref();
		
		if let Some(new_root) = self.find_parent_of_root() {
			self.root_id = self.inner.set_root(new_root);
			
			if let Some(mut new_root) = self.inner.root_mut() {
				Self::resolve_new_root_children(&mut new_root, old_root_path);
			}
			
			Some(self.root_id)
		} else {
			None
		}
	}
	
	fn find_parent_of_root(&mut self) -> Option<FsTreeModelNode> {
		self.inner.get(self.root_id)
		    .and_then(|root| root.data().entry.path())
		    .and_then(Path::parent)
		    .map(FileEntry::from)
		    .map(FsTreeModelNode::from)
	}
	
	fn resolve_new_root_children(new_root: &mut NodeMut<FsTreeModelNode>, old_root_path: Option<&Path>) {
		new_root.data().are_children_known = true;
		
		for child in Self::get_directory_children(&new_root.data().entry).unwrap_or_default() {
			if child.path() != old_root_path {
				new_root.append(FsTreeModelNode::from(child));
			} else if let Some(mut old_root) = new_root.first_child() {
				old_root.make_last_sibling();
			}
		}
	}
}
