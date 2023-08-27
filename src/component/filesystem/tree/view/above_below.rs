use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::tree::{FsTreeView, FsTreeViewNode};

impl FsTreeView {
	pub fn get_node_above(&self, node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> {
		if let Some(prev_sibling_id) = node.prev_sibling_id() {
			Some(self.get_last_descendant_or_self(prev_sibling_id))
		} else {
			node.parent_id()
		}
	}
	
	#[allow(clippy::unused_self)]
	pub fn get_node_below(&self, node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> {
		if let Some(next_id) = node.first_child_id() {
			Some(next_id)
		} else if let Some(next_id) = node.next_sibling_id() {
			Some(next_id)
		} else {
			for ancestor in node.ancestors() {
				if let Some(next_id) = ancestor.next_sibling_id() {
					return Some(next_id);
				}
			}
			None
		}
	}
	
	pub fn get_node_above_id(&self, node_id: NodeId) -> Option<NodeId> {
		self.get(node_id).and_then(|node| self.get_node_above(&node))
	}
	
	pub fn get_node_below_id(&self, node_id: NodeId) -> Option<NodeId> {
		self.get(node_id).and_then(|node| self.get_node_below(&node))
	}
	
	pub fn get_last_descendant_or_self(&self, id: NodeId) -> NodeId {
		let mut id = id;
		
		while let Some(node_id) = self.get(id).and_then(|node| node.last_child_id()) {
			id = node_id;
		}
		
		id
	}
}
