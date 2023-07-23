use std::fs::DirEntry;
use std::io;

use slab_tree::NodeId;

use crate::component::filesystem::tree::{FsTreeModel, FsTreeModelNode};
use crate::file::FileEntry;

impl FsTreeModel {
	pub fn resolve_children(&mut self, node_id: NodeId) -> Option<Vec<NodeId>> {
		if let Some(mut node) = self.get_mut(node_id) {
			let data = node.data();
			
			if !data.are_children_known {
				data.are_children_known = true;
				
				if let Some(children) = Self::get_directory_children(&data.entry) {
					for child in children {
						node.append(FsTreeModelNode::from(child));
					}
				}
			}
			
			node.first_child().map(|node| node.node_id()).map(|id| self.collect_next_siblings(id))
		} else {
			None
		}
	}
	
	pub fn get_children(&self, node_id: NodeId) -> Option<Vec<NodeId>> {
		self.get(node_id).and_then(|node| node.first_child().map(|node| node.node_id()).map(|id| self.collect_next_siblings(id)))
	}
	
	fn collect_next_siblings(&self, first_child_id: NodeId) -> Vec<NodeId> {
		let mut children = Vec::new();
		let mut child_id = Some(first_child_id);
		
		while let Some(child) = child_id.and_then(|id| self.get(id)) {
			children.push(child.node_id());
			child_id = child.next_sibling().map(|node| node.node_id());
		}
		
		children
	}
	
	pub fn get_directory_children(entry: &FileEntry) -> Option<Vec<FileEntry>> {
		entry.path()
		     .and_then(|path| std::fs::read_dir(path).ok())
		     .map(|reader| reader.map(read_entry).collect())
	}
}

#[allow(clippy::needless_pass_by_value)]
fn read_entry(entry: io::Result<DirEntry>) -> FileEntry {
	entry.as_ref().ok().map_or_else(FileEntry::dummy, FileEntry::from)
}
