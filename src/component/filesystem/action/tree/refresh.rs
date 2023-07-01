use crate::component::filesystem::FsLayer;
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;

pub struct RefreshChildrenOfSelected;

impl Action<FsLayer> for RefreshChildrenOfSelected {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		if layer.tree.refresh_children(layer.selected_view_node_id) {
			layer.tree_structure_changed();
			ActionResult::Redraw
		} else {
			ActionResult::Nothing
		}
	}
}
