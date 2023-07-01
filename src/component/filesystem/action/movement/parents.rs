use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::action::movement::MovementAction;
use crate::component::filesystem::FsLayer;
use crate::component::filesystem::tree::{FsTreeView, FsTreeViewNode};
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;

pub struct MoveToParent;

impl MovementAction for MoveToParent {
	fn get_target(_tree: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized {
		selected_node.parent().map(|parent| parent.node_id())
	}
}

pub struct MoveOrTraverseUpParent;

impl Action<FsLayer> for MoveOrTraverseUpParent {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		if let Some(selected_node) = layer.selected_node() {
			if let Some(new_selected_id) = MoveToParent::get_target(&layer.tree.view, &selected_node) {
				layer.selected_view_node_id = new_selected_id;
				return ActionResult::Redraw;
			} else if let Some(new_seelected_id) = layer.tree.traverse_up_root() {
				layer.selected_view_node_id = new_seelected_id;
				layer.tree_structure_changed();
				return ActionResult::Redraw;
			}
		}
		
		ActionResult::Nothing
	}
}
