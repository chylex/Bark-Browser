use slab_tree::{NodeId, NodeMut, RemoveBehavior};

use crate::state::{FileSystemTree, Node};
use crate::state::tree::get_directory_children;

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
	
	if let Some(children) = get_directory_children(&data.entry) {
		for child in children {
			node.append(Node::from(child));
		}
		true
	} else {
		false
	}
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
