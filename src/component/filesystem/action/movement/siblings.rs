use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::action::movement::SimpleMovementAction;
use crate::component::filesystem::tree::{FsTreeView, FsTreeViewNode};
use crate::util::slab_tree::NodeRefExtensions;

pub struct MoveToNextSibling;

impl SimpleMovementAction for MoveToNextSibling {
	fn get_target(view: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized {
		let mut node_id = selected_node.node_id();
		
		while let Some(node) = view.get(node_id) {
			let sibling_id = node.next_sibling_id();
			
			if sibling_id.is_some() {
				return sibling_id;
			} else if let Some(parent_id) = node.parent_id() {
				node_id = parent_id;
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
		selected_node.prev_sibling_id().or_else(|| selected_node.parent_id())
	}
}

pub struct MoveBetweenFirstAndLastSibling;

impl SimpleMovementAction for MoveBetweenFirstAndLastSibling {
	fn get_target(_view: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized {
		if selected_node.next_sibling().is_none() {
			selected_node.parent().and_then(|node| node.first_child_id())
		} else {
			selected_node.parent().and_then(|node| node.last_child_id())
		}
	}
}
