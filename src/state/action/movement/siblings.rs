use slab_tree::{NodeId, NodeRef};

use crate::state::action::movement::MovementAction;
use crate::state::filesystem::{FsTreeView, FsTreeViewNode};

pub struct MoveToNextSibling;

impl MovementAction for MoveToNextSibling {
	fn get_target(_tree: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized {
		selected_node.next_sibling().map(|sibling| sibling.node_id())
	}
}

pub struct MoveToPreviousSibling;

impl MovementAction for MoveToPreviousSibling {
	fn get_target(_tree: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized {
		selected_node.prev_sibling().map(|sibling| sibling.node_id())
	}
}
