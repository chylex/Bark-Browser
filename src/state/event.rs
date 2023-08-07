use crate::state::Environment;

pub trait Event<L> {
	fn dispatch(&self, layer: &mut L, environment: &Environment) -> EventResult;
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum EventResult {
	Nothing,
	Draw,
	Redraw,
}

impl EventResult {
	pub const fn draw_if(condition: bool) -> Self {
		if condition {
			Self::Draw
		} else {
			Self::Nothing
		}
	}
	
	pub fn merge(self, other: Self) -> Self {
		if self == Self::Redraw || other == Self::Redraw {
			Self::Redraw
		} else if self == Self::Draw || other == Self::Draw {
			Self::Draw
		} else {
			Self::Nothing
		}
	}
}
