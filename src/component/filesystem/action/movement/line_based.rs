use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::action::movement::{get_simple_movement_target, MovementAction, MoveToParent, perform_movement_with_count_from, SimpleMovementAction};
use crate::component::filesystem::FsLayer;
use crate::component::filesystem::tree::{FsTreeView, FsTreeViewNode};
use crate::state::Environment;

/// Moves up `count` lines (1 line by default).
pub struct MoveUp;

impl SimpleMovementAction for MoveUp {
	fn get_target(tree: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized {
		tree.get_node_above(selected_node)
	}
}

/// Moves up `count` lines (1 line by default).
pub struct MoveDown;

impl SimpleMovementAction for MoveDown {
	fn get_target(tree: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> {
		tree.get_node_below(selected_node)
	}
}

/// Moves to the first line.
pub struct MoveToFirst;

impl MovementAction for MoveToFirst {
	fn get_target(&self, layer: &mut FsLayer, _environment: &Environment) -> Option<NodeId> where Self: Sized {
		Some(Self::get_target(layer))
	}
}

impl MoveToFirst {
	fn get_target(layer: &mut FsLayer) -> NodeId where Self: Sized {
		let view = &layer.tree.view;
		let mut target_node_id = layer.selected_view_node_id;
		
		while let Some(node_id) = view.get(target_node_id).and_then(|node| <MoveToParent as SimpleMovementAction>::get_target(view, &node)) {
			target_node_id = node_id;
		}
		
		target_node_id
	}
}

/// Moves to the last line.
pub struct MoveToLast;

impl MovementAction for MoveToLast {
	fn get_target(&self, layer: &mut FsLayer, _environment: &Environment) -> Option<NodeId> where Self: Sized {
		let first_node_id = MoveToFirst::get_target(layer);
		let last_node_id = layer.tree.view.get_last_descendant_or_self(first_node_id);
		Some(last_node_id)
	}
}

/// Moves to the line specified by `count` (starting at 1).
/// If no `count` is specified, uses the movement action `A` instead.
pub struct MoveToLineOr<A: MovementAction>(pub A);

impl<A: MovementAction> MovementAction for MoveToLineOr<A> {
	fn get_target(&self, layer: &mut FsLayer, environment: &Environment) -> Option<NodeId> where Self: Sized {
		if let Some(line_number) = layer.registers.count {
			let line_index = Some(line_number.saturating_sub(1));
			let first_node_id = MoveToFirst::get_target(layer);
			Some(perform_movement_with_count_from(layer, line_index, first_node_id, get_simple_movement_target::<MoveDown>))
		} else {
			self.0.get_target(layer, environment)
		}
	}
}
