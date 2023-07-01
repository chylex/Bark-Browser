use std::marker::PhantomData;

use crate::component::filesystem::action::movement::MovementAction;
use crate::component::filesystem::FsLayer;
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;

pub struct RepeatMovement<A: MovementAction + 'static, D: MovementDistance>(D, PhantomData<&'static A>);

impl<A: MovementAction, D: MovementDistance> RepeatMovement<A, D> {
	pub fn new(_action: A, distance: D) -> Self {
		RepeatMovement(distance, PhantomData)
	}
}

impl<A: MovementAction, D: MovementDistance> Action<FsLayer> for RepeatMovement<A, D> {
	fn perform(&self, layer: &mut FsLayer, environment: &Environment) -> ActionResult {
		let view = &layer.tree.view;
		let distance = self.0.get_distance(environment.terminal_height);
		
		let start_node_id = layer.selected_view_node_id;
		let mut target_node_id = start_node_id;
		
		for _ in 0..distance {
			if let Some(id) = view.get(target_node_id).and_then(|node| A::get_target(view, &node)) {
				target_node_id = id;
			} else {
				break;
			}
		}
		
		if target_node_id != start_node_id {
			layer.selected_view_node_id = target_node_id;
			ActionResult::Redraw
		} else {
			ActionResult::Nothing
		}
	}
}

pub trait MovementDistance {
	fn get_distance(&self, terminal_height: u16) -> u16;
}

pub struct HeightRatio(pub u16);

impl MovementDistance for HeightRatio {
	fn get_distance(&self, terminal_height: u16) -> u16 {
		terminal_height / self.0
	}
}
