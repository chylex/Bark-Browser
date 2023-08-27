use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::action::movement::{MovementAction, perform_movement_with_count_from_register, SimpleMovementAction};
use crate::component::filesystem::FsLayer;
use crate::component::filesystem::tree::{FsTree, FsTreeView, FsTreeViewNode};
use crate::state::Environment;

pub struct MoveToNextSibling;

impl SimpleMovementAction for MoveToNextSibling {
	fn get_target(_view: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized {
		selected_node.next_sibling_id()
	}
}

pub struct MoveToPreviousSibling;

impl SimpleMovementAction for MoveToPreviousSibling {
	fn get_target(_view: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized {
		selected_node.prev_sibling_id()
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

pub struct MoveToParent;

impl SimpleMovementAction for MoveToParent {
	fn get_target(_view: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized {
		selected_node.parent_id()
	}
}

pub struct MoveOrTraverseUpParent;

impl MovementAction for MoveOrTraverseUpParent {
	fn get_target(&self, layer: &mut FsLayer, _environment: &Environment) -> Option<NodeId> where Self: Sized {
		Some(perform_movement_with_count_from_register(layer, Self::get_target))
	}
}

impl MoveOrTraverseUpParent {
	fn get_target(tree: &mut FsTree, node_id: NodeId) -> Option<NodeId> {
		if let Some(node) = tree.view.get(node_id) {
			let target_node_id = <MoveToParent as SimpleMovementAction>::get_target(&tree.view, &node);
			if target_node_id.is_some() {
				return target_node_id;
			} else if let Some(target_node_id) = tree.traverse_up_root() {
				return Some(target_node_id)
			}
		}
		
		None
	}
}
