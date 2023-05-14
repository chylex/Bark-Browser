use crate::gui::action::{Action, ActionResult};
use crate::state::State;

pub struct Quit;

impl Action for Quit {
	fn perform(&self, _state: &mut State) -> ActionResult {
		ActionResult::Quit
	}
}
