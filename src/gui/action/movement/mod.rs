use slab_tree::{NodeId, NodeRef};

use crate::gui::action::{Action, ActionResult};
use crate::state::{FileSystemTree, Node, State};

pub use self::parents::MoveOrTraverseUpParent;
pub use self::parents::MoveToParent;
pub use self::up_down::MoveDown;
pub use self::up_down::MoveUp;

mod up_down;
mod parents;

pub trait MovementAction {
	fn get_target(tree: &FileSystemTree, selected_node: &NodeRef<Node>) -> Option<NodeId> where Self: Sized;
}

impl<T: MovementAction> Action for T {
	fn perform(&self, state: &mut State) -> ActionResult {
		return if let Some(next) = state.get_selected_node().and_then(|node| Self::get_target(&state.tree, &node)) {
			state.selected_id = next;
			ActionResult::Redraw
		} else {
			ActionResult::Nothing
		};
	}
}
