use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct KeyBinding {
	code: KeyCode,
	modifiers: KeyModifiers,
}

impl KeyBinding {
	pub const fn new(code: KeyCode, modifiers: KeyModifiers) -> Self {
		Self { code, modifiers }
	}
	
	pub const fn char(char: char) -> Self {
		Self::new(KeyCode::Char(char), KeyModifiers::NONE)
	}
	
	pub const fn code(&self) -> KeyCode {
		self.code
	}
	
	pub const fn modifiers(&self) -> KeyModifiers {
		self.modifiers
	}
}

impl From<KeyEvent> for KeyBinding {
	fn from(key_event: KeyEvent) -> Self {
		let code = key_event.code;
		
		let modifiers = if let KeyCode::Char(_) = code {
			key_event.modifiers & !KeyModifiers::SHIFT // Ignore shift modifier for regular characters.
		} else {
			key_event.modifiers
		};
		
		Self { code, modifiers }
	}
}
