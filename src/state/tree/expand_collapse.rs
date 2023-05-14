use slab_tree::{NodeId, NodeMut, RemoveBehavior};

use crate::file::FileEntry;
use crate::state::{FileSystemTree, Node};

impl FileSystemTree {
	pub fn expand(&mut self, node_id: NodeId) -> bool {
		return if let Some(mut node) = self.get_mut(node_id) {
			expand_node(&mut node)
		} else {
			false
		};
	}
	
	pub fn collapse(&mut self, node_id: NodeId) -> bool {
		return if let Some(mut node) = self.get_mut(node_id) {
			collapse_node(&mut node)
		} else {
			false
		};
	}
	
	pub fn expand_or_collapse(&mut self, node_id: NodeId) -> bool {
		return if let Some(mut node) = self.get_mut(node_id) {
			if node.data().is_expanded {
				collapse_node(&mut node)
			} else {
				expand_node(&mut node)
			}
		} else {
			false
		};
	}
}

fn expand_node(node: &mut NodeMut<Node>) -> bool {
	let data = node.data();
	if data.is_expanded {
		return false;
	}
	
	data.is_expanded = true;
	
	return if let Some(child_entries) = data.entry.path().and_then(|path| std::fs::read_dir(path).ok()) {
		let mut children = child_entries
			.map(|e| e.as_ref().map(FileEntry::from).unwrap_or_else(|_| FileEntry::dummy()))
			.collect::<Vec<_>>();
		
		children.sort_by(|f1, f2| f1.name().cmp(&f2.name()));
		
		for child in children {
			node.append(Node::from(child));
		}
		
		true
	} else {
		false
	};
}

fn collapse_node(node: &mut NodeMut<Node>) -> bool {
	let data = &mut node.data();
	if !data.is_expanded {
		return false;
	}
	
	data.is_expanded = false;
	while node.remove_first(RemoveBehavior::DropChildren).is_some() {}
	
	true
}
