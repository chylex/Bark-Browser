use crate::component::filesystem::FsLayer;
use crate::component::input::InputFieldOverlayLayer;
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

pub struct EnterCommandMode;

impl Action<FsLayer> for EnterCommandMode {
	fn perform(&self, _layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		ActionResult::PushLayer(Box::new(InputFieldOverlayLayer::new(":", Box::new(|command| {
			ActionResult::PopLayer
		}))))
	}
}
