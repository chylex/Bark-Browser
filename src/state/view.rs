use std::io;
use std::io::{stdout, Stdout, Write};

use crossterm::{Command, QueueableCommand};

pub type R = io::Result<()>;

pub struct View {
	out: Stdout,
}

impl View {
	pub fn stdout() -> Self {
		Self { out: stdout() }
	}
	
	pub fn queue(&mut self, command: impl Command) -> R {
		self.out.queue(command)?;
		Ok(())
	}
	
	pub fn flush(&mut self) -> R {
		self.out.flush()
	}
}
