use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::FsLayer;
use crate::component::filesystem::tree::{FsTree, FsTreeViewNode};
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;

pub use self::expand_collapse::*;
pub use self::hierarchy_based::*;
pub use self::line_based::*;
pub use self::with_count::MovementWithCountFactory;
pub use self::with_count::ScreenHeightRatio;
pub use self::with_fallback::MovementWithFallbackFactory;

mod expand_collapse;
mod hierarchy_based;
mod line_based;
mod with_count;
mod with_fallback;

pub trait MovementAction {
	fn get_target(&self, layer: &mut FsLayer, environment: &Environment) -> Option<NodeId> where Self: Sized;
}

impl<T: MovementAction> Action<FsLayer> for T {
	fn perform(&self, layer: &mut FsLayer, environment: &Environment) -> ActionResult {
		if let Some(target_node_id) = self.get_target(layer, environment) {
			layer.tree.selected_view_node_id = target_node_id;
			ActionResult::Draw
		} else {
			ActionResult::Nothing
		}
	}
}

fn perform_movement_with_count_from_register<F>(layer: &mut FsLayer, get_target: F) -> NodeId where F: Fn(&mut FsTree, NodeId) -> Option<NodeId> {
	perform_movement_with_count(&mut layer.tree, layer.registers.count, get_target)
}

fn perform_movement_with_count<F>(tree: &mut FsTree, count: Option<usize>, get_target: F) -> NodeId where F: Fn(&mut FsTree, NodeId) -> Option<NodeId> {
	perform_movement_with_count_from(tree, count, tree.selected_view_node_id, get_target)
}

fn perform_movement_with_count_from<F>(tree: &mut FsTree, count: Option<usize>, start_node_id: NodeId, get_target: F) -> NodeId where F: Fn(&mut FsTree, NodeId) -> Option<NodeId> {
	let mut target_node_id = start_node_id;
	
	for _ in 0..count.unwrap_or(1) {
		if let Some(node_id) = get_target(tree, target_node_id) {
			target_node_id = node_id;
		} else {
			break;
		}
	}
	
	target_node_id
}

pub trait SimpleMovementAction {
	fn get_target(selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized;
}

impl<T: SimpleMovementAction> MovementAction for T {
	fn get_target(&self, layer: &mut FsLayer, _environment: &Environment) -> Option<NodeId> where Self: Sized {
		Some(perform_movement_with_count_from_register(layer, get_simple_movement_target::<T>))
	}
}

fn get_simple_movement_target<T: SimpleMovementAction>(tree: &mut FsTree, node_id: NodeId) -> Option<NodeId> {
	tree.get_view_node(node_id).and_then(|node| T::get_target(&node))
}
