#[derive(Copy, Clone)]
pub enum FileMode {
	Known(u32),
	Unknown,
}

impl FileMode {
	pub fn user(&self) -> PermissionClassMode {
		self.get_class(2)
	}
	
	pub fn group(&self) -> PermissionClassMode {
		self.get_class(1)
	}
	
	pub fn others(&self) -> PermissionClassMode {
		self.get_class(0)
	}
	
	pub fn is_setuid(&self) -> Option<bool> {
		self.get_bit(11)
	}
	
	pub fn is_setgid(&self) -> Option<bool> {
		self.get_bit(10)
	}
	
	pub fn is_sticky(&self) -> Option<bool> {
		self.get_bit(9)
	}
	
	fn get_class(&self, group_index: u8) -> PermissionClassMode {
		PermissionClassMode::from(self.get_bits(group_index * 3, 0b111))
	}
	
	fn get_bit(&self, bit_index: u8) -> Option<bool> {
		self.get_bits(bit_index, 1).map(|b| b != 0)
	}
	
	fn get_bits(&self, shift: u8, mask: u8) -> Option<u8> {
		if let FileMode::Known(mode) = self {
			let shift32 = shift as u32;
			let mask32 = mask as u32;
			Some(((*mode >> shift32) & mask32) as u8)
		} else {
			None
		}
	}
}

#[derive(Copy, Clone)]
pub enum PermissionClassMode {
	Known(u8),
	Unknown
}

impl PermissionClassMode {
	fn test_permission(&self, mask: u8) -> Permission {
		if let PermissionClassMode::Known(mode) = self {
			if mode & mask != 0 {
				Permission::Yes
			} else {
				Permission::No
			}
		} else {
			Permission::Unknown
		}
	}
	
	pub fn read(&self) -> Permission {
		self.test_permission(0b100)
	}
	
	pub fn write(&self) -> Permission {
		self.test_permission(0b010)
	}
	
	pub fn execute(&self) -> Permission {
		self.test_permission(0b001)
	}
}

impl From<Option<u8>> for PermissionClassMode {
	fn from(mode: Option<u8>) -> Self {
		if let Some(mode) = mode {
			Self::Known(mode)
		} else {
			Self::Unknown
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Permission {
	Yes,
	No,
	Unknown
}
