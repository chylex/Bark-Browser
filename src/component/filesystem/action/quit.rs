use crate::component::filesystem::FsLayer;
use crate::state::action::{Action, ActionResult};

pub struct Quit;

impl Action<FsLayer> for Quit {
	fn perform(&self, _layer: &mut FsLayer) -> ActionResult {
		ActionResult::PopLayer
	}
}
