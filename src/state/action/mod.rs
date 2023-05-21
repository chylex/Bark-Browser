use crate::state::State;

pub use self::quit::Quit;

mod quit;
pub mod movement;
pub mod tree;

pub trait Action {
	fn perform(&self, state: &mut State) -> ActionResult;
}

pub enum ActionResult {
	Nothing,
	Redraw { tree_structure_changed: bool },
	Quit,
}

impl ActionResult {
	pub fn redraw() -> Self {
		Self::Redraw { tree_structure_changed: false }
	}
}
