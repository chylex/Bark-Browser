use std::cmp::min;

use ratatui::layout::{Alignment, Margin, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, BorderType, Clear, Padding};

use crate::state::view::F;

pub mod message;

const MARGIN_HORIZONTAL: u16 = 1;
const MARGIN_VERTICAL: u16 = 0;

const PADDING_HORIZONTAL: u16 = 3;
const PADDING_VERTICAL: u16 = 1;

fn calculate_margin_area(frame: &mut F, content_width: u16, content_height: u16) -> Rect {
	let frame_size = frame.size();
	
	let x = 0;
	let width = min(content_width.saturating_add((MARGIN_HORIZONTAL * 2) + 2 + (PADDING_HORIZONTAL * 2)), frame_size.width);
	let height = min(content_height.saturating_add((MARGIN_VERTICAL * 2) + 2 + (PADDING_VERTICAL * 2)), frame_size.height);
	let y = (frame_size.height - height) / 2;
	
	Rect { x, y, width, height }
}

fn render_dialog_border<'a, T>(frame: &mut F, content_width: u16, content_height: u16, title: T, color: Color) -> Rect where T: Into<Line<'a>> {
	let margin_area = calculate_margin_area(frame, content_width, content_height);
	let border_area = margin_area.inner(&Margin { horizontal: MARGIN_HORIZONTAL, vertical: MARGIN_VERTICAL });
	
	let border_widget = Block::default()
		.title(title)
		.title_alignment(Alignment::Center)
		.borders(Borders::ALL)
		.border_type(BorderType::Plain)
		.border_style(Style::default().fg(color))
		.padding(Padding::new(PADDING_HORIZONTAL, PADDING_HORIZONTAL, PADDING_VERTICAL, PADDING_VERTICAL));
	
	let content_area = border_widget.inner(border_area);
	
	frame.render_widget(Clear, margin_area);
	frame.render_widget(border_widget, border_area);
	
	content_area
}
