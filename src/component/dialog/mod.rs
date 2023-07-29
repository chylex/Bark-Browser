use std::cmp::min;

use ratatui::layout::{Alignment, Margin, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, BorderType, Clear, Padding};

use crate::state::view::Frame;

pub mod input;
pub mod message;

const MARGIN_HORIZONTAL: u16 = 1;
const MARGIN_VERTICAL: u16 = 0;

const PADDING_HORIZONTAL: u16 = 3;
const PADDING_VERTICAL: u16 = 1;

fn calculate_margin_area(frame: &mut Frame, top_y: u16, content_width: u16, content_height: u16) -> Rect {
	let frame_size = frame.size();
	
	let width = min(content_width.saturating_add((MARGIN_HORIZONTAL * 2) + 2 + (PADDING_HORIZONTAL * 2)), frame_size.width);
	let height = min(content_height.saturating_add((MARGIN_VERTICAL * 2) + 2 + (PADDING_VERTICAL * 2)), frame_size.height);
	
	let x = 0;
	let y = min(top_y, frame_size.height.saturating_sub(height));
	
	Rect { x, y, width, height }
}

fn render_dialog_border<'a>(frame: &mut Frame, top_y: u16, content_width: u16, content_height: u16, title: impl Into<Line<'a>>, color: Color) -> Rect {
	let margin_area = calculate_margin_area(frame, top_y, content_width, content_height);
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
