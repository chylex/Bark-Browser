use crate::component::filesystem::FsLayer;
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;

pub struct ExpandCollapse;

impl Action<FsLayer> for ExpandCollapse {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		if layer.tree.expand_or_collapse(layer.selected_view_node_id) {
			layer.tree_structure_changed();
			ActionResult::Redraw
		} else {
			ActionResult::Nothing
		}
	}
}
