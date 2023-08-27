use slab_tree::NodeId;

use crate::component::filesystem::action::movement::{get_simple_movement_target, MovementAction, perform_movement_with_count_from_register, SimpleMovementAction};
use crate::component::filesystem::FsLayer;
use crate::component::filesystem::tree::FsTree;
use crate::state::Environment;

pub struct ExpandSelectedOr<M: SimpleMovementAction>(pub M);

impl<M: SimpleMovementAction> MovementAction for ExpandSelectedOr<M> {
	fn get_target(&self, layer: &mut FsLayer, _environment: &Environment) -> Option<NodeId> where Self: Sized {
		Some(perform_action_or_movement::<M, _>(layer, FsTree::expand))
	}
}

pub struct CollapseSelectedOr<M: SimpleMovementAction>(pub M);

impl<M: SimpleMovementAction> MovementAction for CollapseSelectedOr<M> {
	fn get_target(&self, layer: &mut FsLayer, _environment: &Environment) -> Option<NodeId> where Self: Sized {
		Some(perform_action_or_movement::<M, _>(layer, FsTree::collapse))
	}
}

fn perform_action_or_movement<M: SimpleMovementAction, F>(layer: &mut FsLayer, action: F) -> NodeId where F: Fn(&mut FsTree, NodeId) -> bool {
	perform_movement_with_count_from_register(layer, |tree, node_id| {
		if action(tree, node_id) {
			Some(node_id)
		} else {
			get_simple_movement_target::<M>(tree, node_id)
		}
	})
}
