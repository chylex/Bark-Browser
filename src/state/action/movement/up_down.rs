use slab_tree::{NodeId, NodeRef};

use crate::state::action::movement::MovementAction;
use crate::state::filesystem::{FsTreeView, FsTreeViewNode};

pub struct MoveUp;

impl MovementAction for MoveUp {
	fn get_target(tree: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized {
		tree.get_node_above(selected_node)
	}
}

pub struct MoveDown;

impl MovementAction for MoveDown {
	fn get_target(tree: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> {
		tree.get_node_below(selected_node)
	}
}
