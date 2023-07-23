use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::tree::{FsTreeView, FsTreeViewNode};

impl FsTreeView {
	pub fn get_node_above(&self, node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> {
		if let Some(prev_sibling) = node.prev_sibling() {
			Some(self.get_last_descendant_or_self(prev_sibling.node_id()))
		} else {
			node.parent().map(|parent| parent.node_id())
		}
	}
	
	#[allow(clippy::unused_self)]
	pub fn get_node_below(&self, node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> {
		if let Some(next) = node.first_child() {
			Some(next.node_id())
		} else if let Some(next) = node.next_sibling() {
			Some(next.node_id())
		} else {
			for ancestor in node.ancestors() {
				if let Some(next) = ancestor.next_sibling() {
					return Some(next.node_id());
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
		
		while let Some(node_id) = self.get(id).and_then(|node| node.last_child().map(|last_child| last_child.node_id())) {
			id = node_id;
		}
		
		id
	}
}
