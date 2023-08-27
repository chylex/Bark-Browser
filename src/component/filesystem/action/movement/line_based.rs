use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::action::movement::{get_simple_movement_target, MovementAction, MoveToParent, perform_movement_with_count_from, SimpleMovementAction};
use crate::component::filesystem::FsLayer;
use crate::component::filesystem::tree::{FsTree, FsTreeView, FsTreeViewNode};
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
		Some(Self::get_target(&mut layer.tree))
	}
}

impl MoveToFirst {
	fn get_target(tree: &mut FsTree) -> NodeId where Self: Sized {
		let mut target_node_id = tree.selected_view_node_id;
		
		while let Some(node_id) = tree.view.get(target_node_id).and_then(|node| <MoveToParent as SimpleMovementAction>::get_target(&tree.view, &node)) {
			target_node_id = node_id;
		}
		
		target_node_id
	}
}

/// Moves to the last line.
pub struct MoveToLast;

impl MovementAction for MoveToLast {
	fn get_target(&self, layer: &mut FsLayer, _environment: &Environment) -> Option<NodeId> where Self: Sized {
		let first_node_id = MoveToFirst::get_target(&mut layer.tree);
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
			let tree = &mut layer.tree;
			let line_index = Some(line_number.saturating_sub(1));
			let first_node_id = MoveToFirst::get_target(tree);
			Some(perform_movement_with_count_from(tree, line_index, first_node_id, get_simple_movement_target::<MoveDown>))
		} else {
			self.0.get_target(layer, environment)
		}
	}
}
