use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::tree::{FsTreeView, FsTreeViewNode};

impl FsTreeView {
	pub fn get_node_above(&self, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> {
		if let Some(prev_sibling) = selected_node.prev_sibling() {
			Some(self.get_last_descendant_or_self(prev_sibling.node_id()))
		} else {
			selected_node.parent().map(|parent| parent.node_id())
		}
	}
	
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
	
	fn get_last_descendant_or_self(&self, id: NodeId) -> NodeId {
		let mut id = id;
		
		while let Some(node_id) = self.get(id).and_then(|node| node.last_child().map(|last_child| last_child.node_id())) {
			id = node_id;
		}
		
		id
	}
}
