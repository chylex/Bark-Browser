pub trait IntegerLength {
	fn int_len(self) -> usize;
}

impl IntegerLength for u64 {
	fn int_len(self) -> usize {
		self.checked_ilog10().unwrap_or(0).saturating_add(1) as usize
	}
}

impl IntegerLength for u32 {
	fn int_len(self) -> usize {
		self.checked_ilog10().unwrap_or(0).saturating_add(1) as usize
	}
}

impl IntegerLength for i32 {
	fn int_len(self) -> usize {
		let sign_len = if self < 0 { 1 } else { 0 };
		let digit_count = self.abs().checked_ilog10().unwrap_or(1).saturating_add(1);
		digit_count.saturating_add(sign_len) as usize
	}
}

impl<T> IntegerLength for &T where T: IntegerLength + Copy {
	fn int_len(self) -> usize {
		T::int_len(*self)
	}
}

pub fn int_len<T : IntegerLength>(value: T) -> usize {
	value.int_len()
}
