pub struct FsTreeRegisters {
	pub count: Option<usize>,
}

impl FsTreeRegisters {
	pub fn new() -> Self {
		Self {
			count: None,
		}
	}
}
