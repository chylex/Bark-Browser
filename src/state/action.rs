use crate::state::Environment;
use crate::state::layer::Layer;

pub trait Action<L> {
	fn perform(&self, layer: &mut L, environment: &Environment) -> ActionResult;
}

pub enum ActionResult {
	Nothing,
	Draw,
	Redraw,
	PushLayer(Box<dyn Layer>),
	ReplaceLayer(Box<dyn Layer>),
	PopLayer,
}

impl ActionResult {
	pub const fn draw_if(condition: bool) -> Self {
		if condition {
			Self::Draw
		} else {
			Self::Nothing
		}
	}
}
