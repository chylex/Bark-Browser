use std::str::Chars;

use crossterm::event::{KeyCode, KeyModifiers};

use crate::input::keymap::binding::KeyBinding;

pub struct KeySequenceParser<'a> {
	chars: Chars<'a>,
}

impl<'a> KeySequenceParser<'a> {
	pub fn new(key_sequence: &'a str) -> Self {
		Self { chars: key_sequence.chars() }
	}
	
	pub fn next(&mut self) -> Result<Option<KeyBinding>, ParseError> {
		if let Some(char) = self.chars.next() {
			if char == '<' {
				self.read_special()
			} else {
				Ok(Some(KeyBinding::char(char)))
			}
		} else {
			Ok(None)
		}
	}
	
	fn read_special(&mut self) -> Result<Option<KeyBinding>, ParseError> {
		let mut modifiers = KeyModifiers::NONE;
		let mut current_part = String::new();
		
		for char in self.chars.by_ref() {
			if char == '>' {
				let code = parse_key_name(current_part.as_str())?;
				return Ok(Some(KeyBinding::new(code, modifiers)));
			} else if char == '-' {
				modifiers |= match current_part.as_str() {
					"A" => KeyModifiers::ALT,
					"C" => KeyModifiers::CONTROL,
					"M" => KeyModifiers::SUPER,
					"S" => KeyModifiers::SHIFT,
					_ => return Err(ParseError::InvalidModifier),
				};
				current_part.clear();
			} else {
				current_part.push(char);
			}
		}
		
		Err(ParseError::MissingClosingAngledBracket)
	}
}

fn parse_key_name(key: &str) -> Result<KeyCode, ParseError> {
	match key {
		"BS" => Ok(KeyCode::Backspace),
		"Bar" => Ok(KeyCode::Char('|')),
		"Bslash" => Ok(KeyCode::Char('\\')),
		"Del" => Ok(KeyCode::Delete),
		"Down" => Ok(KeyCode::Down),
		"End" => Ok(KeyCode::End),
		"Enter" => Ok(KeyCode::Enter),
		"Esc" => Ok(KeyCode::Esc),
		"F1" => Ok(KeyCode::F(1)),
		"F2" => Ok(KeyCode::F(2)),
		"F3" => Ok(KeyCode::F(3)),
		"F4" => Ok(KeyCode::F(4)),
		"F5" => Ok(KeyCode::F(5)),
		"F6" => Ok(KeyCode::F(6)),
		"F7" => Ok(KeyCode::F(7)),
		"F8" => Ok(KeyCode::F(8)),
		"F9" => Ok(KeyCode::F(9)),
		"F10" => Ok(KeyCode::F(10)),
		"F11" => Ok(KeyCode::F(11)),
		"F12" => Ok(KeyCode::F(12)),
		"Home" => Ok(KeyCode::Home),
		"Insert" => Ok(KeyCode::Insert),
		"Left" => Ok(KeyCode::Left),
		"lt" => Ok(KeyCode::Char('<')),
		"PageDown" => Ok(KeyCode::PageDown),
		"PageUp" => Ok(KeyCode::PageUp),
		"Return" => Ok(KeyCode::Enter),
		"Right" => Ok(KeyCode::Right),
		"Space" => Ok(KeyCode::Char(' ')),
		"Tab" => Ok(KeyCode::Tab),
		"Up" => Ok(KeyCode::Up),
		
		_ => {
			let chars = key.chars().collect::<Vec<_>>();
			if chars.len() == 1 {
				Ok(KeyCode::Char(chars[0]))
			} else {
				Err(ParseError::InvalidKeyName)
			}
		},
	}
}

#[derive(Debug, Copy, Clone)]
pub enum ParseError {
	InvalidKeyName,
	InvalidModifier,
	MissingClosingAngledBracket,
}
