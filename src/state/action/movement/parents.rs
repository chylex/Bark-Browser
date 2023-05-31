use slab_tree::{NodeId, NodeRef};

use crate::state::action::{Action, ActionResult};
use crate::state::action::movement::MovementAction;
use crate::state::filesystem::{FsTreeView, FsTreeViewNode};
use crate::state::State;

pub struct MoveToParent;

impl MovementAction for MoveToParent {
	fn get_target(_tree: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized {
		selected_node.parent().map(|parent| parent.node_id())
	}
}

pub struct MoveOrTraverseUpParent;

impl Action for MoveOrTraverseUpParent {
	fn perform(&self, state: &mut State) -> ActionResult {
		if let Some(selected_node) = state.selected_node() {
			if let Some(new_selected_id) = MoveToParent::get_target(&state.tree.view, &selected_node) {
				state.selected_view_node_id = new_selected_id;
				return ActionResult::redraw();
			} else if let Some(new_seelected_id) = state.tree.traverse_up_root() {
				state.selected_view_node_id = new_seelected_id;
				return ActionResult::Redraw { tree_structure_changed: true };
			}
		}
		
		ActionResult::Nothing
	}
}
