use ratatui::style::Color;
use ratatui::text::{Line, Text};

use crate::component::dialog::input::InputFieldDialogLayer;
use crate::state::action::ActionResult;

pub struct InputFieldDialogBuilder;

pub struct InputFieldDialogBuilder1 {
	y: u16,
}

pub struct InputFieldDialogBuilder2 {
	step1: InputFieldDialogBuilder1,
	min_width: u16,
}

pub struct InputFieldDialogBuilder3 {
	step2: InputFieldDialogBuilder2,
	default_color: Color,
	darker_color: Color,
}

pub struct InputFieldDialogBuilder4<'a> {
	step3: InputFieldDialogBuilder3,
	title: Line<'a>,
}

pub struct InputFieldDialogBuilder5<'a> {
	step4: InputFieldDialogBuilder4<'a>,
	message: Text<'a>,
	initial_value: Option<String>,
}

impl InputFieldDialogBuilder {
	#[allow(clippy::unused_self)]
	pub const fn y(self, y: u16) -> InputFieldDialogBuilder1 {
		InputFieldDialogBuilder1 { y }
	}
}

impl InputFieldDialogBuilder1 {
	pub const fn min_width(self, min_width: u16) -> InputFieldDialogBuilder2 {
		InputFieldDialogBuilder2 { step1: self, min_width }
	}
}

impl InputFieldDialogBuilder2 {
	pub const fn color(self, default_color: Color, darker_color: Color) -> InputFieldDialogBuilder3 {
		InputFieldDialogBuilder3 { step2: self, default_color, darker_color }
	}
}

impl InputFieldDialogBuilder3 {
	pub fn title<'a>(self, title: impl Into<Line<'a>>) -> InputFieldDialogBuilder4<'a> {
		InputFieldDialogBuilder4 { step3: self, title: title.into() }
	}
}

impl<'a> InputFieldDialogBuilder4<'a> {
	pub fn message(self, message: impl Into<Text<'a>>) -> InputFieldDialogBuilder5<'a> {
		InputFieldDialogBuilder5 { step4: self, message: message.into(), initial_value: None }
	}
}

impl<'a> InputFieldDialogBuilder5<'a> {
	pub fn initial_value(mut self, initial_value: Option<impl Into<String>>) -> Self {
		self.initial_value = initial_value.map(Into::into);
		self
	}
	
	pub fn build(self, confirm_action: Box<dyn Fn(String) -> ActionResult>) -> InputFieldDialogLayer<'a> {
		let step4 = self.step4;
		let step3 = step4.step3;
		let step2 = step3.step2;
		let step1 = step2.step1;
		InputFieldDialogLayer::new(step1.y, step2.min_width, step3.default_color, step3.darker_color, step4.title, self.message, self.initial_value, confirm_action)
	}
}
