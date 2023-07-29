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
}

impl View {
	pub fn stdout() -> io::Result<Self> {
		terminal::enable_raw_mode()?;
		
		let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
		
		term.backend_mut().execute(terminal::EnterAlternateScreen)?;
		term.hide_cursor()?;
		term.clear()?;
		
		Ok(Self { term })
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
	
	pub fn clear(&mut self) -> io::Result<()> {
		self.term.clear()
	}
	
	pub fn render<R>(&mut self, renderer: R) -> io::Result<CompletedFrame> where R: FnOnce(&mut Frame) {
		self.term.draw(|frame| {
			let mut frame = Frame::new(frame);
			renderer(&mut frame);
			frame.apply_cursor();
		})
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
