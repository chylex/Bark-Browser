use std::cmp::min;

use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Clear, Paragraph};

use crate::input::keymap::KeyBinding;
use crate::state::action::ActionResult;
use crate::state::Environment;
use crate::state::layer::Layer;
use crate::state::view::F;

pub struct InputFieldLayer {
	text: String,
	confirm_action: Box<dyn Fn(String) -> ActionResult>,
}

impl InputFieldLayer {
	pub fn new(confirm_action: Box<dyn Fn(String) -> ActionResult>) -> Self {
		Self { text: String::new(), confirm_action }
	}
}

impl Layer for InputFieldLayer {
	fn handle_input(&mut self, _environment: &Environment, key_binding: KeyBinding) -> ActionResult {
		match key_binding.code() {
			KeyCode::Esc => {
				ActionResult::PopLayer
			}
			
			KeyCode::Enter => {
				(self.confirm_action)(self.text.clone())
			}
			
			KeyCode::Backspace => {
				self.text.pop();
				ActionResult::Draw
			}
			
			KeyCode::Char(c) => {
				match key_binding.modifiers() {
					KeyModifiers::CONTROL => {
						ActionResult::PopLayer
					}
					
					_ => {
						self.text.push(c);
						ActionResult::Draw
					}
				}
			}
			
			_ => {
				ActionResult::Nothing
			}
		}
	}
	
	fn render(&mut self, frame: &mut F) {
		let size = frame.size();
		let area = Rect::new(size.x, size.bottom() - 1, size.width, 1);
		
		let text = Text::from(self.text.clone());
		let text_width = u16::try_from(text.width()).unwrap_or(u16::MAX);
		let text_offset = text_width.saturating_sub(size.width.saturating_sub(1));
		
		let style = Style::default()
			.fg(Color::Black)
			.bg(Color::LightYellow);
		
		let para = Paragraph::new(text)
			.style(style)
			.scroll((0, text_offset));
		
		frame.render_widget(Clear, area);
		frame.render_widget(para, area);
		frame.set_cursor(min(area.x.saturating_add(text_width), size.width), area.y);
	}
}
