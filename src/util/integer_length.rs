pub trait IntegerLength {
	fn int_len(self) -> usize;
}

impl IntegerLength for u32 {
	fn int_len(self) -> usize {
		(self.checked_ilog10().unwrap_or(0) + 1) as usize
	}
}

impl IntegerLength for i32 {
	fn int_len(self) -> usize {
		let digit_count = self.abs().checked_ilog10().unwrap_or(1) + 1;
		let sign_len = if self < 0 { 1 } else { 0 };
		(digit_count + sign_len) as usize
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
