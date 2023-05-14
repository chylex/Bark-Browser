use slab_tree::{NodeId, NodeRef};

use crate::gui::action::movement::MovementAction;
use crate::state::{FileSystemTree, Node};

pub struct MoveDown;

impl MovementAction for MoveDown {
	fn get_target(tree: &FileSystemTree, selected_node: &NodeRef<Node>) -> Option<NodeId> {
		tree.get_node_below(selected_node)
	}
}

pub struct MoveUp;

impl MovementAction for MoveUp {
	fn get_target(tree: &FileSystemTree, selected_node: &NodeRef<Node>) -> Option<NodeId> where Self: Sized {
		tree.get_node_above(selected_node)
	}
}