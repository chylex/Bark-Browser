use slab_tree::{NodeId, NodeRef};

use crate::state::action::{Action, ActionResult};
use crate::state::filesystem::{FsTreeView, FsTreeViewNode};
use crate::state::State;

pub use self::parents::MoveOrTraverseUpParent;
pub use self::parents::MoveToParent;
pub use self::siblings::MoveToNextSibling;
pub use self::siblings::MoveToPreviousSibling;
pub use self::up_down::MoveDown;
pub use self::up_down::MoveUp;

mod parents;
mod siblings;
mod up_down;

pub trait MovementAction {
	fn get_target(tree: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized;
}

impl<T: MovementAction> Action for T {
	fn perform(&self, state: &mut State) -> ActionResult {
		return if let Some(next) = state.selected_node().and_then(|node| Self::get_target(&state.tree.view, &node)) {
			state.selected_view_node_id = next;
			ActionResult::redraw()
		} else {
			ActionResult::Nothing
		};
	}
}
