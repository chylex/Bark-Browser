use crate::state::layer::Layer;

pub trait Action<L> {
	fn perform(&self, layer: &mut L) -> ActionResult;
}

pub enum ActionResult {
	Nothing,
	Redraw,
	PushLayer(Box<dyn Layer>),
	ReplaceLayer(Box<dyn Layer>),
	PopLayer,
}
