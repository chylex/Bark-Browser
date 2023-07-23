pub struct FsTreeRegisters {
	pub count: Option<usize>,
}

impl FsTreeRegisters {
	pub const fn new() -> Self {
		Self {
			count: None,
		}
	}
}
