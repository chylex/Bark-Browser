use std::ffi::OsString;
use std::fmt::{Display, Formatter};

use unicode_segmentation::UnicodeSegmentation;

pub struct FileName {
	os: Option<OsString>,
	str: String,
}

impl FileName {
	pub fn dummy() -> Self {
		FileName::from("???")
	}
	
	pub fn str(&self) -> &str {
		&self.str
	}
	
	pub fn len(&self) -> usize {
		self.str.graphemes(true).count()
	}
}

impl From<OsString> for FileName {
	fn from(value: OsString) -> Self {
		let str = value.to_string_lossy().to_string();
		let os = Some(value);
		
		Self { os, str }
	}
}

impl From<&str> for FileName {
	fn from(value: &str) -> Self {
		Self {
			os: None,
			str: value.to_string(),
		}
	}
}

impl Display for FileName {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.str)
	}
}
