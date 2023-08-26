use ratatui::style::Color;
use ratatui::text::{Line, Text};

use crate::component::dialog::message::{MessageDialogActionMap, MessageDialogLayer};
use crate::component::dialog::message::actions::MessageDialogActions;
use crate::state::action::ActionResult;

pub struct MessageDialogBuilder;

pub struct MessageDialogBuilder1 {
	y: u16,
}

pub struct MessageDialogBuilder2 {
	step1: MessageDialogBuilder1,
	color: Color,
}

pub struct MessageDialogBuilder3<'a> {
	step2: MessageDialogBuilder2,
	title: Line<'a>,
}

pub struct MessageDialogBuilder4<'a> {
	step3: MessageDialogBuilder3<'a>,
	message: Text<'a>,
}

impl MessageDialogBuilder {
	#[allow(clippy::unused_self)]
	pub const fn y(self, y: u16) -> MessageDialogBuilder1 {
		MessageDialogBuilder1 { y }
	}
}

impl MessageDialogBuilder1 {
	pub const fn color(self, color: Color) -> MessageDialogBuilder2 {
		MessageDialogBuilder2 { step1: self, color }
	}
}

impl MessageDialogBuilder2 {
	pub fn title<'a>(self, title: impl Into<Line<'a>>) -> MessageDialogBuilder3<'a> {
		MessageDialogBuilder3 { step2: self, title: title.into() }
	}
}

impl<'a> MessageDialogBuilder3<'a> {
	pub fn message(self, message: impl Into<Text<'a>>) -> MessageDialogBuilder4<'a> {
		MessageDialogBuilder4 { step3: self, message: message.into() }
	}
}

impl<'a> MessageDialogBuilder4<'a> {
	pub fn actions<A>(self, actions: A) -> MessageDialogLayer<'a> where A: MessageDialogActions<'a> + 'a {
		let step3 = self.step3;
		let step2 = step3.step2;
		let step1 = step2.step1;
		MessageDialogLayer::new(step1.y, step2.color, step3.title, self.message, actions)
	}
	
	pub fn ok(self) -> MessageDialogLayer<'a> {
		self.actions(MessageDialogActionMap::ok())
	}
	
	pub fn yes_no<F>(self, yes_action: F) -> MessageDialogLayer<'a> where F: Fn() -> ActionResult + 'static {
		self.actions(MessageDialogActionMap::yes_no(yes_action))
	}
}
