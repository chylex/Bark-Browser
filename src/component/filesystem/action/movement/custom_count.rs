use std::marker::PhantomData;

use slab_tree::NodeId;

use crate::component::filesystem::action::movement::{get_simple_movement_target, MovementAction, perform_movement_with_count, SimpleMovementAction};
use crate::component::filesystem::FsLayer;
use crate::state::Environment;

/// A movement action with a custom count.
pub struct MovementWithCount<A: SimpleMovementAction + 'static, C: MovementCount>(C, PhantomData<&'static A>);

/// Defines custom count for a movement action.
pub trait MovementCount {
	/// Computes the custom count for a movement action.
	fn get_count(&self, original_count: Option<usize>, environment: &Environment) -> usize;
}

/// Utility trait for creating movement actions with a custom count out of [SimpleMovementAction] implementations.
pub trait MovementWithCountFactory<A: SimpleMovementAction + 'static, C: MovementCount> {
	/// Creates a movement action that always uses a custom count.
	fn with_custom_count(self, count: C) -> MovementWithCount<A, C>;
	
	/// Creates a movement action that uses a custom count if the user has not explicitly specified one.
	fn with_default_count(self, default_count: C) -> MovementWithCount<A, DefaultCount<C>>;
}

impl<A: SimpleMovementAction + 'static, C: MovementCount> MovementWithCountFactory<A, C> for A {
	fn with_custom_count(self, count: C) -> MovementWithCount<A, C> {
		MovementWithCount(count, PhantomData)
	}
	
	fn with_default_count(self, default_count: C) -> MovementWithCount<A, DefaultCount<C>> {
		MovementWithCount(DefaultCount(default_count), PhantomData)
	}
}

impl<A: SimpleMovementAction, C: MovementCount> MovementAction for MovementWithCount<A, C> {
	fn get_target(&self, layer: &mut FsLayer, environment: &Environment) -> Option<NodeId> where Self: Sized {
		let count = self.0.get_count(layer.registers.count, environment);
		Some(perform_movement_with_count(layer, Some(count), get_simple_movement_target::<A>))
	}
}

/// Delegates to the custom count if the user has not explicitly specified one.
pub struct DefaultCount<C: MovementCount>(pub C);

impl<C: MovementCount> MovementCount for DefaultCount<C> {
	fn get_count(&self, original_count: Option<usize>, environment: &Environment) -> usize {
		original_count.unwrap_or_else(|| self.0.get_count(None, environment))
	}
}

/// Defines movement count as the terminal height divided by a constant.
pub struct ScreenHeightRatio(pub usize);

impl MovementCount for ScreenHeightRatio {
	fn get_count(&self, original_count: Option<usize>, environment: &Environment) -> usize {
		let terminal_height = environment.terminal_height as usize;
		let height_ratio = terminal_height / self.0;
		original_count.unwrap_or(1) * (height_ratio)
	}
}
