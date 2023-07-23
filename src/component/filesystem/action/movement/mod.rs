use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::FsLayer;
use crate::component::filesystem::tree::{FsTreeView, FsTreeViewNode};
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;

pub use self::custom_count::MovementWithCountFactory;
pub use self::custom_count::ScreenHeightRatio;
pub use self::parents::MoveOrTraverseUpParent;
pub use self::siblings::MoveToNextSibling;
pub use self::siblings::MoveToPreviousSibling;
pub use self::up_down::MoveDown;
pub use self::up_down::MoveUp;

mod custom_count;
mod parents;
mod siblings;
mod up_down;

pub trait MovementAction {
	fn get_target(&self, layer: &mut FsLayer, environment: &Environment) -> Option<NodeId> where Self: Sized;
}

impl<T: MovementAction> Action<FsLayer> for T {
	fn perform(&self, layer: &mut FsLayer, environment: &Environment) -> ActionResult {
		if let Some(target_node_id) = self.get_target(layer, environment) {
			layer.selected_view_node_id = target_node_id;
			ActionResult::Draw
		} else {
			ActionResult::Nothing
		}
	}
}

fn perform_movement_with_count<F>(layer: &mut FsLayer, count: Option<usize>, get_target: F) -> NodeId where F: Fn(&mut FsLayer, NodeId) -> Option<NodeId> {
	let mut target_node_id = layer.selected_view_node_id;
	
	for _ in 0..count.unwrap_or(1) {
		if let Some(id) = get_target(layer, target_node_id) {
			target_node_id = id;
		} else {
			break;
		}
	}
	
	target_node_id
}

pub trait SimpleMovementAction {
	fn get_target(view: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized;
}

impl<T: SimpleMovementAction> MovementAction for T {
	fn get_target(&self, layer: &mut FsLayer, _environment: &Environment) -> Option<NodeId> where Self: Sized {
		Some(perform_movement_with_count(layer, layer.registers.count, get_simple_movement_target::<T>))
	}
}

fn get_simple_movement_target<T: SimpleMovementAction>(layer: &mut FsLayer, node_id: NodeId) -> Option<NodeId> {
	let view = &layer.tree.view;
	view.get(node_id).and_then(|node| T::get_target(view, &node))
}
