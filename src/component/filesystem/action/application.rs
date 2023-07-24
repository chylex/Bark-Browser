use crate::component::filesystem::FsLayer;
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;

pub struct Quit;

impl Action<FsLayer> for Quit {
	fn perform(&self, _layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		ActionResult::PopLayer
	}
}

pub struct RedrawScreen;

impl Action<FsLayer> for RedrawScreen {
	fn perform(&self, _layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		ActionResult::Redraw
	}
}
