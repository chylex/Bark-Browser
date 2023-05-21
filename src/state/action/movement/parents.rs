use slab_tree::{NodeId, NodeRef};

use crate::state::{FileSystemTree, Node, State};
use crate::state::action::{Action, ActionResult};
use crate::state::action::movement::MovementAction;

pub struct MoveToParent;

impl MovementAction for MoveToParent {
	fn get_target(_tree: &FileSystemTree, selected_node: &NodeRef<Node>) -> Option<NodeId> where Self: Sized {
		selected_node.parent().map(|parent| parent.node_id())
	}
}

pub struct MoveOrTraverseUpParent;

impl Action for MoveOrTraverseUpParent {
	fn perform(&self, state: &mut State) -> ActionResult {
		if let Some(new_selected_id) = state.get_selected_node().and_then(|node| MoveToParent::get_target(&state.tree, &node)).or_else(|| state.tree.traverse_up_root()) {
			state.selected_id = new_selected_id;
			return ActionResult::redraw();
		}
		
		ActionResult::Nothing
	}
}
