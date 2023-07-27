use std::borrow::Cow;
use std::cmp::min;

use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Clear, Paragraph, StatefulWidget, Widget};

use crate::input::keymap::KeyBinding;
use crate::state::view::F;

pub use self::overlay::InputFieldOverlayLayer;

mod overlay;

pub struct InputField {
	text: String,
	caret: usize,
}

impl InputField {
	pub const fn new() -> Self {
		Self {
			text: String::new(),
			caret: 0,
		}
	}
	
	fn text(&self) -> &str {
		&self.text
	}
	
	fn caret_end(&self) -> usize {
		self.text.chars().count()
	}
	
	fn insert_at_caret(&mut self, c: char) -> bool {
		if self.text.len() >= u16::MAX as usize {
			false
		} else {
			let insert_at = self.text.char_indices().nth(self.caret).map(|(i, _)| i).unwrap_or_else(|| self.text.len());
			self.text.insert(insert_at, c);
			self.caret = self.caret.saturating_add(1);
			true
		}
	}
	
	fn delete_at_caret(&mut self) -> bool {
		if let Some((delete_at, _)) = self.text.char_indices().nth(self.caret.saturating_sub(1)) {
			self.text.remove(delete_at);
			self.caret = self.caret.saturating_sub(1);
			true
		} else {
			false
		}
	}
	
	fn delete_in_front_of_caret(&mut self) -> bool {
		if let Some((delete_at, _)) = self.text.char_indices().nth(self.caret) {
			self.text.remove(delete_at);
			true
		} else {
			false
		}
	}
	
	fn clear(&mut self) -> bool {
		if self.text.is_empty() {
			false
		} else {
			self.text.clear();
			self.caret = 0;
			true
		}
	}
	
	#[allow(clippy::wildcard_enum_match_arm)]
	pub fn handle_input(&mut self, key_binding: KeyBinding) -> bool {
		match (key_binding.code(), key_binding.modifiers()) {
			(KeyCode::Left, KeyModifiers::NONE) => {
				self.caret = self.caret.saturating_sub(1);
				true
			}
			
			(KeyCode::Right, KeyModifiers::NONE) => {
				self.caret = min(self.caret.saturating_add(1), self.caret_end());
				true
			}
			
			(KeyCode::Home, KeyModifiers::NONE) => {
				self.caret = 0;
				true
			}
			
			(KeyCode::End, KeyModifiers::NONE) => {
				self.caret = self.caret_end();
				true
			}
			
			(KeyCode::Delete, KeyModifiers::NONE) => {
				self.delete_in_front_of_caret()
			}
			
			(KeyCode::Backspace, KeyModifiers::NONE) => {
				self.delete_at_caret()
			}
			
			(KeyCode::Char('u'), KeyModifiers::CONTROL) => {
				self.clear()
			}
			
			(KeyCode::Char(c), KeyModifiers::NONE) => {
				self.insert_at_caret(c)
			}
			
			_ => false
		}
	}
	
	pub fn render(&mut self, frame: &mut F, x: u16, y: u16, width: u16) {
		let area = Rect::new(x, y, width, 1);
		
		let widget = InputFieldWidget {
			text: self.text(),
			caret: self.caret,
		};
		
		let mut caret_x = 0;
		frame.render_stateful_widget(widget, area, &mut caret_x);
		frame.set_cursor(area.x.saturating_add(caret_x), area.y);
	}
}

struct InputFieldWidget<'a> {
	text: &'a str,
	caret: usize,
}

impl<'a> StatefulWidget for InputFieldWidget<'a> {
	type State = u16;
	
	fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
		if area.height < 1 {
			return;
		}
		
		let mut remaining_width = area.width.saturating_sub(1) as usize;
		let mut char_indices_before_caret = Vec::new();
		let mut caret_x = 0_usize;
		
		let mut start_char_index = 0;
		let mut end_char_index = self.text.len();
		let mut has_truncated_end = false;
		
		for (index, (char_index, char)) in self.text.char_indices().enumerate() {
			let char_str = &mut [0; 4];
			let char_str = char.encode_utf8(char_str);
			
			let char_width = Span::raw(Cow::Borrowed(char_str)).width();
			let next_char_index = char_index.saturating_add(char_str.len());
			
			// Ensure the last character before caret is always visible.
			if self.caret.checked_sub(1).is_some_and(|before_caret| index == before_caret) {
				start_char_index = char_index;
				caret_x = char_width;
				remaining_width = remaining_width.saturating_sub(char_width);
				continue;
			}
			
			// Remember characters before caret for later.
			if index < self.caret {
				char_indices_before_caret.push((char_index, char_width));
				continue;
			}
			
			// Append characters at / after caret until we run out of space.
			if let Some(new_remaining_width) = remaining_width.checked_sub(char_width) {
				remaining_width = new_remaining_width;
				end_char_index = next_char_index;
				continue;
			} else {
				has_truncated_end = true;
				break;
			}
		}
		
		// Prepend characters before caret until we run out of space.
		for (char_index, char_width) in char_indices_before_caret.into_iter().rev() {
			if let Some(new_remaining_width) = remaining_width.checked_sub(char_width) {
				remaining_width = new_remaining_width;
			} else {
				break;
			}
			
			caret_x = caret_x.saturating_add(char_width);
			start_char_index = char_index;
		}
		
		let style = Style::default()
			.fg(Color::Black)
			.bg(Color::LightYellow);
		
		Clear.render(area, buf);
		
		#[allow(clippy::indexing_slicing, clippy::string_slice)] // Indices are obtained from char_indices.
		Paragraph::new(&self.text[start_char_index..end_char_index])
			.style(style)
			.render(area, buf);
		
		if has_truncated_end {
			buf.get_mut(area.right().saturating_sub(1), area.y)
			   .set_char('~')
			   .set_fg(Color::DarkGray)
			   .set_bg(Color::Yellow);
		}
		
		*state = u16::try_from(caret_x).unwrap_or(u16::MAX);
	}
}
