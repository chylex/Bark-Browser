use ratatui::layout::Alignment;
use ratatui::style::Color;
use ratatui::text::{Line, Text};
use ratatui::widgets::Paragraph;

use actions::MessageDialogActions;

use crate::component::dialog::message::builder::MessageDialogBuilder;
use crate::component::dialog::render_dialog_border;
use crate::input::keymap::KeyBinding;
use crate::state::action::ActionResult;
use crate::state::Environment;
use crate::state::layer::Layer;
use crate::state::view::Frame;

pub use self::actions::MessageDialogActionMap;

mod actions;
mod builder;

pub struct MessageDialogLayer<'a> {
	y: u16,
	color: Color,
	title: Line<'a>,
	message: Text<'a>,
	actions: Box<dyn MessageDialogActions<'a> + 'a>,
}

impl<'a> MessageDialogLayer<'a> {
	fn new<A>(y: u16, color: Color, title: Line<'a>, mut message: Text<'a>, actions: A) -> Self where A: MessageDialogActions<'a> + 'a {
		let action_line = Line::from(actions.describe().clone()).alignment(Alignment::Right);
		let actions = Box::new(actions);
		
		message.lines.push(action_line);
		
		Self { y, color, title, message, actions }
	}
	
	pub const fn build() -> MessageDialogBuilder {
		MessageDialogBuilder
	}
	
	pub fn generic_error(y: u16, message: impl Into<Text<'a>>) -> MessageDialogLayer<'a> {
		Self::build()
			.y(y)
			.color(Color::LightRed)
			.title("Error")
			.message(message)
			.ok()
	}
}

impl Layer for MessageDialogLayer<'_> {
	fn handle_input(&mut self, _environment: &Environment, key_binding: KeyBinding) -> ActionResult {
		self.actions.handle_input(key_binding)
	}
	
	fn render(&mut self, frame: &mut Frame) {
		let content_width = u16::try_from(self.message.width()).unwrap_or(u16::MAX);
		let content_height = u16::try_from(self.message.height()).unwrap_or(u16::MAX);
		
		let paragraph = Paragraph::new(self.message.clone()).alignment(Alignment::Left);
		let content_area = render_dialog_border(frame, self.y, content_width, content_height, self.title.clone(), self.color);
		
		frame.render_widget(paragraph, content_area);
	}
}
