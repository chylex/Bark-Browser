use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;

use crate::component::input::InputField;
use crate::input::keymap::KeyBinding;
use crate::state::action::ActionResult;
use crate::state::Environment;
use crate::state::layer::Layer;
use crate::state::view::Frame;

pub struct InputFieldOverlayLayer<'a> {
	field: InputField,
	read_only_prefix: &'a str,
	confirm_action: Box<dyn Fn(String) -> ActionResult>,
}

impl<'a> InputFieldOverlayLayer<'a> {
	pub fn new(read_only_prefix: &'a str, confirm_action: Box<dyn Fn(String) -> ActionResult>) -> Self {
		Self {
			field: InputField::new(),
			read_only_prefix,
			confirm_action,
		}
	}
}

impl<'a> Layer for InputFieldOverlayLayer<'a> {
	#[allow(clippy::wildcard_enum_match_arm)]
	fn handle_input(&mut self, _environment: &Environment, key_binding: KeyBinding) -> ActionResult {
		match (key_binding.code(), key_binding.modifiers()) {
			(KeyCode::Esc, KeyModifiers::NONE) |
			(KeyCode::Char('c'), KeyModifiers::CONTROL) => {
				ActionResult::PopLayer
			}
			
			(KeyCode::Enter, KeyModifiers::NONE) => {
				(self.confirm_action)(self.field.text().to_owned())
			}
			
			(KeyCode::Backspace, KeyModifiers::NONE) => {
				if self.field.text().is_empty() {
					ActionResult::PopLayer
				} else {
					ActionResult::draw_if(self.field.handle_input(key_binding))
				}
			}
			
			_ => {
				ActionResult::draw_if(self.field.handle_input(key_binding))
			}
		}
	}
	
	fn render(&mut self, frame: &mut Frame) {
		let size = frame.size();
		if size.width < 1 || size.height < 1 {
			return;
		}
		
		let x = size.x;
		let y = size.bottom().saturating_sub(1);
		
		let prefix_style = Style::new()
			.fg(Color::Black)
			.bg(Color::LightYellow);
		
		let prefix_paragraph = Paragraph::new(self.read_only_prefix)
			.style(prefix_style);
		
		frame.render_widget(prefix_paragraph, Rect { x, y, width: 1, height: 1 });
		
		if size.width > 1 {
			self.field.render(frame, x.saturating_add(1), y, size.width.saturating_sub(1), Color::LightYellow, Color::Yellow);
		}
	}
}
