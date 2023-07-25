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
use crate::state::view::F;

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
	fn new<T, M, A>(y: u16, color: Color, title: T, message: M, actions: A) -> Self where T: Into<Line<'a>>, M: Into<Text<'a>>, A: MessageDialogActions<'a> + 'a {
		let mut message = message.into();
		message.lines.push(Line::from(actions.describe().clone()).alignment(Alignment::Right));
		
		let title = title.into();
		let actions = Box::new(actions);
		
		Self { y, color, title, message, actions }
	}
	
	pub const fn build() -> MessageDialogBuilder {
		MessageDialogBuilder
	}
	
	pub fn generic_error<M>(y: u16, message: M) -> MessageDialogLayer<'a> where M: Into<Text<'a>> {
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
	
	fn render(&mut self, frame: &mut F) {
		let content_width = u16::try_from(self.message.width()).unwrap_or(u16::MAX);
		let content_height = u16::try_from(self.message.height()).unwrap_or(u16::MAX);
		
		let paragraph = Paragraph::new(self.message.clone()).alignment(Alignment::Center);
		let content_area = render_dialog_border(frame, self.y, content_width, content_height, self.title.clone(), self.color);
		
		frame.render_widget(paragraph, content_area);
	}
}
