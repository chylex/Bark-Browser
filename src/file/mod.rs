use std::ffi::OsStr;
use std::fs::{DirEntry, Metadata};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use lazy_static::lazy_static;
use normalize_path::NormalizePath;

pub use crate::file::kind::FileKind;
pub use crate::file::mode::{FileMode, Permission};
pub use crate::file::name::FileName;
pub use crate::file::owner::{FileOwner, FileOwnerName, FileOwnerNameCache};

mod kind;
mod mode;
mod name;
mod owner;

pub struct FileEntry {
	path: Option<PathBuf>,
	name: FileName,
	kind: FileKind,
	mode: FileMode,
	owner: Option<FileOwner>,
	mtime: Option<SystemTime>,
}

lazy_static! {
	static ref DUMMY: FileEntry = FileEntry::dummy();
}

impl FileEntry {
	fn new(path: impl AsRef<Path>, name: FileName, metadata: Result<&Metadata, &std::io::Error>) -> Self {
		let path = path.as_ref();
		assert!(path.is_absolute(), "Path is not absolute: {path:?}");
		
		Self {
			path: Some(path.normalize()),
			name,
			kind: metadata.map(FileKind::from).unwrap_or(FileKind::Unknown),
			mode: metadata.map(FileMode::from).unwrap_or(FileMode::Unknown),
			owner: metadata.ok().and_then(|m| FileOwner::try_from(m).ok()),
			mtime: metadata.ok().and_then(|m| m.modified().ok()),
		}
	}
	
	pub fn dummy() -> Self {
		Self {
			path: None,
			name: FileName::dummy(),
			kind: FileKind::Unknown,
			mode: FileMode::Unknown,
			owner: None,
			mtime: None,
		}
	}
	
	pub fn dummy_as_ref() -> &'static Self {
		&DUMMY
	}
	
	pub fn path(&self) -> Option<&Path> {
		self.path.as_deref()
	}
	
	pub const fn name(&self) -> &FileName {
		&self.name
	}
	
	pub const fn kind(&self) -> &FileKind {
		&self.kind
	}
	
	pub const fn mode(&self) -> FileMode {
		self.mode
	}
	
	pub fn uid(&self) -> Option<u32> {
		self.owner.map(FileOwner::uid)
	}
	
	pub fn gid(&self) -> Option<u32> {
		self.owner.map(FileOwner::gid)
	}
	
	pub const fn modified_time(&self) -> Option<&SystemTime> {
		self.mtime.as_ref()
	}
}

impl From<&DirEntry> for FileEntry {
	fn from(entry: &DirEntry) -> Self {
		let path = entry.path();
		let name = FileName::from(entry.file_name());
		let metadata = entry.metadata();
		return Self::new(path, name, metadata.as_ref());
	}
}

impl From<&Path> for FileEntry {
	fn from(path: &Path) -> Self {
		let name = if path == Path::new("/") {
			FileName::from("/")
		} else {
			path.file_name().map(OsStr::to_os_string).map_or_else(FileName::dummy, FileName::from)
		};
		
		let metadata = path.symlink_metadata();
		return Self::new(path, name, metadata.as_ref());
	}
}
