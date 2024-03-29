use std::collections::HashMap;

use ratatui::style::{Color, Style};
use ratatui::text::Span;

use crate::input::keymap::KeyBinding;
use crate::state::action::ActionResult;

type ActionHashMap = HashMap<KeyBinding, Box<dyn Fn() -> ActionResult>>;

pub trait MessageDialogActions<'a> {
	fn handle_input(&mut self, key_binding: KeyBinding) -> ActionResult;
	fn describe(&self) -> &Vec<Span<'a>>;
}

pub struct MessageDialogActionMap<'a> {
	map: ActionHashMap,
	description: Vec<Span<'a>>,
}

impl<'a> MessageDialogActionMap<'a> {
	fn new(map: ActionHashMap, description: Vec<Span<'a>>) -> Self {
		Self { map, description }
	}
	
	fn highlight() -> Style {
		Style::default().fg(Color::LightCyan)
	}
	
	pub fn ok() -> Self {
		let mut map = ActionHashMap::new();
		map.insert(KeyBinding::char('o'), Box::new(|| ActionResult::PopLayer));
		
		Self::new(map, vec![
			Span::styled("o", Self::highlight()),
			Span::raw("k"),
		])
	}
	
	pub fn yes_no<F>(yes_action: F) -> Self where F: Fn() -> ActionResult + 'static {
		let mut map = ActionHashMap::new();
		map.insert(KeyBinding::char('y'), Box::new(yes_action));
		map.insert(KeyBinding::char('n'), Box::new(|| ActionResult::PopLayer));
		
		Self::new(map, vec![
			Span::styled("y", Self::highlight()),
			Span::raw("es/"),
			Span::styled("n", Self::highlight()),
			Span::raw("o"),
		])
	}
}

impl<'a> MessageDialogActions<'a> for MessageDialogActionMap<'a> {
	fn handle_input(&mut self, key_binding: KeyBinding) -> ActionResult {
		self.map.get(&key_binding).map(|f| f()).unwrap_or(ActionResult::Nothing)
	}
	
	fn describe(&self) -> &Vec<Span<'a>> {
		&self.description
	}
}
