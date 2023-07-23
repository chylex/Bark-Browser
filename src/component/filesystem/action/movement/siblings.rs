use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::action::movement::SimpleMovementAction;
use crate::component::filesystem::tree::{FsTreeView, FsTreeViewNode};

pub struct MoveToNextSibling;

impl SimpleMovementAction for MoveToNextSibling {
	fn get_target(view: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized {
		let mut node_id = selected_node.node_id();
		
		while let Some(node) = view.get(node_id) {
			let sibling_id = node.next_sibling().map(|sibling| sibling.node_id());
			
			if sibling_id.is_some() {
				return sibling_id;
			} else if let Some(parent) = node.parent() {
				node_id = parent.node_id();
			} else {
				return None;
			}
		}
		
		None
	}
}

pub struct MoveToPreviousSibling;

impl SimpleMovementAction for MoveToPreviousSibling {
	fn get_target(_view: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized {
		selected_node.prev_sibling().or_else(|| selected_node.parent()).map(|node| node.node_id())
	}
}
