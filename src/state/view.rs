use std::{io, panic};
use std::io::{stdout, Stdout};

use crossterm::{ExecutableCommand, terminal};
use ratatui::{Frame, Terminal};
use ratatui::backend::CrosstermBackend;
use ratatui::terminal::CompletedFrame;

pub type F<'a> = Frame<'a, CrosstermBackend<Stdout>>;

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
	
	pub fn render<R>(&mut self, renderer: R) -> io::Result<CompletedFrame> where R: FnOnce(&mut F) {
		self.term.draw(renderer)
	}
}
