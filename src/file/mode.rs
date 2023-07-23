use std::fs::Metadata;

#[derive(Copy, Clone)]
pub enum FileMode {
	Known(u32),
	Unknown,
}

impl FileMode {
	pub fn user(self) -> PermissionClassMode {
		self.get_class(2)
	}
	
	pub fn group(self) -> PermissionClassMode {
		self.get_class(1)
	}
	
	pub fn others(self) -> PermissionClassMode {
		self.get_class(0)
	}
	
	pub fn is_executable_by_any(self) -> Option<bool> {
		self.get_bits(0, 0b_001_001_001).map(|b| b != 0)
	}
	
	pub fn is_setuid(self) -> Option<bool> {
		self.get_bit(11)
	}
	
	pub fn is_setgid(self) -> Option<bool> {
		self.get_bit(10)
	}
	
	pub fn is_sticky(self) -> Option<bool> {
		self.get_bit(9)
	}
	
	fn get_class(self, group_index: u8) -> PermissionClassMode {
		#[allow(clippy::arithmetic_side_effects)] // Only used with constants where overflow is impossible.
		PermissionClassMode::from(self.get_bits(group_index * 3, 0b111))
	}
	
	fn get_bit(self, bit_index: u8) -> Option<bool> {
		self.get_bits(bit_index, 1).map(|b| b != 0)
	}
	
	const fn get_bits(self, shift: u8, mask: u8) -> Option<u8> {
		if let Self::Known(mode) = self {
			let shift32 = shift as u32;
			let mask32 = mask as u32;
			let bits = (mode >> shift32) & mask32;
			#[allow(clippy::cast_possible_truncation)] // All bits above u8 are already masked out.
			Some(bits as u8)
		} else {
			None
		}
	}
}

impl From<&Metadata> for FileMode {
	#[cfg(unix)]
	fn from(metadata: &Metadata) -> Self {
		use std::os::unix::fs::MetadataExt;
		
		Self::Known(metadata.mode())
	}
	
	#[cfg(not(unix))]
	fn from(_metadata: &Metadata) -> Self {
		Self::Unknown
	}
}

#[derive(Copy, Clone)]
pub enum PermissionClassMode {
	Known(u8),
	Unknown
}

impl PermissionClassMode {
	const fn test_permission(self, mask: u8) -> Permission {
		if let Self::Known(mode) = self {
			if Self::has_bit(mode, mask) {
				Permission::Yes
			} else {
				Permission::No
			}
		} else {
			Permission::Unknown
		}
	}
	
	const fn has_bit(mode: u8, mask: u8) -> bool {
		mode & mask != 0
	}
	
	pub const fn read(self) -> Permission {
		self.test_permission(0b100)
	}
	
	pub const fn write(self) -> Permission {
		self.test_permission(0b010)
	}
	
	pub const fn execute(self) -> Permission {
		self.test_permission(0b001)
	}
}

impl From<Option<u8>> for PermissionClassMode {
	fn from(mode: Option<u8>) -> Self {
		mode.map_or(Self::Unknown, Self::Known)
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Permission {
	Yes,
	No,
	Unknown
}
