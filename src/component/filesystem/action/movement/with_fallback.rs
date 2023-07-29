use std::marker::PhantomData;

use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::action::movement::SimpleMovementAction;
use crate::component::filesystem::tree::{FsTreeView, FsTreeViewNode};

/// A movement action with fallback.
pub struct MovementWithFallback<A: SimpleMovementAction, F: SimpleMovementAction>(PhantomData<A>, PhantomData<F>);

/// Utility trait for creating movement actions with fallback out of [`SimpleMovementAction`] implementations.
pub trait MovementWithFallbackFactory<A: SimpleMovementAction, F: SimpleMovementAction> {
	/// Creates a movement action with a fallback movement.
	fn with_fallback(&self, fallback: F) -> MovementWithFallback<A, F>;
}

impl<A: SimpleMovementAction, F: SimpleMovementAction> MovementWithFallbackFactory<A, F> for A {
	fn with_fallback(&self, _fallback: F) -> MovementWithFallback<A, F> {
		MovementWithFallback(PhantomData::<A>, PhantomData::<F>)
	}
}

impl<A: SimpleMovementAction, F: SimpleMovementAction> SimpleMovementAction for MovementWithFallback<A, F> {
	fn get_target(view: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized {
		A::get_target(view, selected_node).or_else(|| F::get_target(view, selected_node))
	}
}
