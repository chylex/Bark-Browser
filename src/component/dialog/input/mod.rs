use std::cmp::max;

use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::Alignment;
use ratatui::style::Color;
use ratatui::text::{Line, Text};
use ratatui::widgets::Paragraph;

use crate::component::dialog::input::builder::InputFieldDialogBuilder;
use crate::component::dialog::render_dialog_border;
use crate::component::input::InputField;
use crate::input::keymap::KeyBinding;
use crate::state::action::ActionResult;
use crate::state::Environment;
use crate::state::layer::Layer;
use crate::state::view::Frame;

mod builder;

pub struct InputFieldDialogLayer<'a> {
	y: u16,
	min_width: u16,
	default_color: Color,
	darker_color: Color,
	title: Line<'a>,
	message: Text<'a>,
	field: InputField,
	confirm_action: Box<dyn Fn(String) -> ActionResult>,
}

impl<'a> InputFieldDialogLayer<'a> {
	fn new(y: u16, min_width: u16, default_color: Color, darker_color: Color, title: Line<'a>, message: Text<'a>, initial_value: Option<String>, confirm_action: Box<dyn Fn(String) -> ActionResult>) -> Self {
		let field = initial_value.map_or_else(InputField::new, InputField::with_text);
		Self { y, min_width, default_color, darker_color, title, message, field, confirm_action }
	}
	
	pub const fn build() -> InputFieldDialogBuilder {
		InputFieldDialogBuilder
	}
}

impl<'a> Layer for InputFieldDialogLayer<'a> {
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
			
			_ => {
				ActionResult::draw_if(self.field.handle_input(key_binding))
			}
		}
	}
	
	fn render(&mut self, frame: &mut Frame) {
		let message_width = u16::try_from(self.message.width()).unwrap_or(u16::MAX);
		let message_height = u16::try_from(self.message.height()).unwrap_or(u16::MAX);
		
		let content_width = max(message_width, self.min_width);
		let content_height = message_height.saturating_add(2);
		
		let paragraph = Paragraph::new(self.message.clone()).alignment(Alignment::Left);
		let content_area = render_dialog_border(frame, self.y, content_width, content_height, self.title.clone(), self.default_color);
		
		frame.render_widget(paragraph, content_area);
		self.field.render(frame, content_area.x, content_area.bottom().saturating_sub(1), content_area.width, self.default_color, self.darker_color);
	}
}
