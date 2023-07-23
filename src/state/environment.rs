use std::io;

use crate::state::view::View;

pub struct Environment {
	pub terminal_width: u16,
	pub terminal_height: u16,
}

impl TryFrom<&View> for Environment {
	type Error = io::Error;
	
	fn try_from(view: &View) -> Result<Self, Self::Error> {
		let size = view.size()?;
		
		Ok(Self {
			terminal_width: size.width,
			terminal_height: size.height,
		})
	}
}
