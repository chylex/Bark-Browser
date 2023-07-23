use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::action::movement::SimpleMovementAction;
use crate::component::filesystem::tree::{FsTreeView, FsTreeViewNode};

pub struct MoveUp;

impl SimpleMovementAction for MoveUp {
	fn get_target(tree: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized {
		tree.get_node_above(selected_node)
	}
}

pub struct MoveDown;

impl SimpleMovementAction for MoveDown {
	fn get_target(tree: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> {
		tree.get_node_below(selected_node)
	}
}
