use crate::gui::action::{Action, ActionResult};
use crate::state::State;

pub struct ExpandCollapse;

impl Action for ExpandCollapse {
	fn perform(&self, state: &mut State) -> ActionResult {
		if state.tree.expand_or_collapse(state.selected_id) {
			ActionResult::Redraw
		} else {
			ActionResult::Nothing
		}
	}
}
