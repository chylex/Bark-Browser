use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::FsLayer;
use crate::component::filesystem::tree::{FsTreeView, FsTreeViewNode};
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;

pub use self::parents::MoveOrTraverseUpParent;
pub use self::parents::MoveToParent;
pub use self::repeat::HeightRatio;
pub use self::repeat::RepeatMovement;
pub use self::siblings::MoveToNextSibling;
pub use self::siblings::MoveToPreviousSibling;
pub use self::up_down::MoveDown;
pub use self::up_down::MoveUp;

mod parents;
mod siblings;
mod up_down;
mod repeat;

pub trait MovementAction {
	fn get_target(tree: &FsTreeView, selected_node: &NodeRef<FsTreeViewNode>) -> Option<NodeId> where Self: Sized;
}

impl<T: MovementAction> Action<FsLayer> for T {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		if let Some(next) = layer.selected_node().and_then(|node| Self::get_target(&layer.tree.view, &node)) {
			layer.selected_view_node_id = next;
			ActionResult::Draw
		} else {
			ActionResult::Nothing
		}
	}
}
