use std::{io, panic};
use std::io::{stdout, Stdout};

use crossterm::{ExecutableCommand, terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Rect;
use ratatui::Terminal;
use ratatui::terminal::CompletedFrame;
use ratatui::widgets::{StatefulWidget, Widget};

pub struct View {
	term: Terminal<CrosstermBackend<Stdout>>,
	render_request: RenderRequest,
}

impl View {
	pub fn stdout() -> io::Result<Self> {
		terminal::enable_raw_mode()?;
		
		let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
		
		term.backend_mut().execute(terminal::EnterAlternateScreen)?;
		term.hide_cursor()?;
		term.clear()?;
		
		Ok(Self { term, render_request: RenderRequest::Draw })
	}
	
	pub fn restore_terminal_on_panic() {
		let prev_hook = panic::take_hook();
		
		panic::set_hook(Box::new(move |panic_info| {
			let _ = terminal::disable_raw_mode();
			prev_hook(panic_info);
		}));
	}
	
	pub fn close(mut self) -> io::Result<()> {
		self.term.show_cursor()?;
		self.term.backend_mut().execute(terminal::LeaveAlternateScreen)?;
		
		terminal::disable_raw_mode()
	}
	
	pub fn size(&self) -> io::Result<Rect> {
		self.term.size()
	}
	
	pub fn set_dirty(&mut self, full_redraw: bool) {
		let new_request = if full_redraw {
			RenderRequest::Redraw
		} else {
			RenderRequest::Draw
		};
		
		self.render_request = self.render_request.merge(new_request);
	}
	
	pub fn render<R>(&mut self, renderer: R) -> io::Result<()> where R: FnOnce(&mut Frame) {
		match self.render_request.consume() {
			RenderRequest::Skip => {}
			
			RenderRequest::Draw => {
				self.draw(renderer)?;
			}
			
			RenderRequest::Redraw => {
				self.term.clear()?;
				self.draw(renderer)?;
			}
		}
		
		Ok(())
	}
	
	fn draw<R>(&mut self, renderer: R) -> io::Result<CompletedFrame> where R: FnOnce(&mut Frame) {
		self.term.draw(|frame| {
			let mut frame = Frame::new(frame);
			renderer(&mut frame);
			frame.apply_cursor();
		})
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum RenderRequest {
	Skip,
	Draw,
	Redraw,
}

impl RenderRequest {
	fn merge(self, other: Self) -> Self {
		if self == Self::Redraw || other == Self::Redraw {
			Self::Redraw
		} else if self == Self::Draw || other == Self::Draw {
			Self::Draw
		} else {
			Self::Skip
		}
	}
	
	fn consume(&mut self) -> Self {
		std::mem::replace(self, Self::Skip)
	}
}

pub struct Frame<'a, 'b> {
	inner: &'a mut ratatui::Frame<'b, CrosstermBackend<Stdout>>,
	cursor: Option<(u16, u16)>,
}

impl<'a, 'b> Frame<'a, 'b> {
	pub fn new(inner: &'a mut ratatui::Frame<'b, CrosstermBackend<Stdout>>) -> Self {
		Self { inner, cursor: None }
	}
	
	pub fn size(&self) -> Rect {
		self.inner.size()
	}
	
	pub fn render_widget<W: Widget>(&mut self, widget: W, area: Rect) {
		self.inner.render_widget(widget, area);
	}
	
	pub fn render_stateful_widget<W: StatefulWidget>(&mut self, widget: W, area: Rect, state: &mut W::State) {
		self.inner.render_stateful_widget(widget, area, state);
	}
	
	pub fn set_cursor(&mut self, x: u16, y: u16) {
		self.cursor = Some((x, y));
	}
	
	pub fn hide_cursor(&mut self) {
		self.cursor = None;
	}
	
	fn apply_cursor(&mut self) {
		if let Some((x, y)) = self.cursor {
			self.inner.set_cursor(x, y);
		}
	}
}
