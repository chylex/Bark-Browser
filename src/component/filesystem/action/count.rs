use ratatui::style::Color;

use crate::component::dialog::message::{MessageDialogActionMap, MessageDialogLayer};
use crate::component::filesystem::FsLayer;
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;

pub struct PushCountDigit(pub u8);

const MAX_COUNT: usize = 99_999;

impl Action<FsLayer> for PushCountDigit {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		let next_digit = self.0 as usize;
		
		let old_count = layer.registers.count.unwrap_or(0);
		let new_count = old_count.saturating_mul(10).saturating_add(next_digit);
		
		if new_count > MAX_COUNT {
			layer.registers.count = None;
			ActionResult::PushLayer(Box::new(MessageDialogLayer::new(Color::LightRed, "Error", format!("Count is too large (> {MAX_COUNT}), it will be reset."), MessageDialogActionMap::ok())))
		} else {
			layer.registers.count = Some(new_count);
			ActionResult::Nothing
		}
	}
}
