use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::Metadata;

use crate::util;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FileOwner {
	uid: u32,
	gid: u32,
}

impl FileOwner {
	pub fn uid(&self) -> u32 {
		self.uid
	}
	
	pub fn gid(&self) -> u32 {
		self.gid
	}
}

impl TryFrom<&Metadata> for FileOwner {
	type Error = ();
	
	#[cfg(unix)]
	fn try_from(metadata: &Metadata) -> Result<Self, Self::Error> {
		use std::os::unix::fs::MetadataExt;
		
		Ok(Self {
			uid: metadata.uid(),
			gid: metadata.gid(),
		})
	}
	
	#[cfg(not(unix))]
	fn try_from(_metadata: &Metadata) -> Result<Self, Self::Error> {
		Err(())
	}
}

pub enum FileOwnerName {
	Named(String),
	Numeric(u32),
	Unknown,
}

impl FileOwnerName {
	pub fn len(&self) -> usize {
		match self {
			Self::Named(name) => name.len(),
			Self::Numeric(id) => util::int_len(id),
			Self::Unknown => 3,
		}
	}
}

impl Display for FileOwnerName {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Named(name) => write!(f, "{}", name),
			Self::Numeric(id) => write!(f, "{}", id),
			Self::Unknown => write!(f, "???"),
		}
	}
}

pub struct FileOwnerNameCache {
	user_name_cache: HashMap<u32, FileOwnerName>,
	group_name_cache: HashMap<u32, FileOwnerName>,
}

impl FileOwnerNameCache {
	pub fn new() -> Self {
		Self {
			user_name_cache: HashMap::new(),
			group_name_cache: HashMap::new(),
		}
	}
	
	pub fn get_user(&mut self, uid: Option<u32>) -> &FileOwnerName {
		Self::get_name_with_cache(&mut self.user_name_cache, uid, system::get_user_name_by_uid)
	}
	
	pub fn get_group(&mut self, gid: Option<u32>) -> &FileOwnerName {
		Self::get_name_with_cache(&mut self.group_name_cache, gid, system::get_group_name_by_gid)
	}
	
	fn get_name_with_cache<F>(cache: &mut HashMap<u32, FileOwnerName>, id: Option<u32>, get_name: F) -> &FileOwnerName where F: FnOnce(u32) -> Option<String> {
		if let Some(id) = id {
			cache.entry(id).or_insert_with_key(|&id| {
				match get_name(id) {
					Some(name) => FileOwnerName::Named(name),
					None => FileOwnerName::Numeric(id),
				}
			})
		} else {
			&FileOwnerName::Unknown
		}
	}
}

#[cfg(unix)]
mod system {
	pub fn get_user_name_by_uid(uid: u32) -> Option<String> {
		users::get_user_by_uid(uid).and_then(|user| user.name().to_str().map(str::to_string))
	}
	
	pub fn get_group_name_by_gid(gid: u32) -> Option<String> {
		users::get_group_by_gid(gid).and_then(|group| group.name().to_str().map(str::to_string))
	}
}

#[cfg(not(unix))]
mod system {
	pub fn get_user_name_by_uid(uid: u32) -> Option<String> {
		None
	}
	
	pub fn get_group_name_by_gid(gid: u32) -> Option<String> {
		None
	}
}
