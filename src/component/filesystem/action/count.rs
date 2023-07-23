use crate::component::filesystem::FsLayer;
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;

pub struct PushCountDigit(pub u8);

impl Action<FsLayer> for PushCountDigit {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		let next_digit = self.0 as usize;
		
		let old_count = layer.registers.count.unwrap_or(0);
		layer.registers.count = Some(old_count * 10 + next_digit);
		
		ActionResult::Nothing
	}
}
