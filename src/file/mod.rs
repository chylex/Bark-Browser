use std::ffi::OsStr;
use std::fs::{DirEntry, Metadata};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub use crate::file::kind::FileKind;
pub use crate::file::mode::{FileMode, Permission, PermissionClassMode};
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

impl FileEntry {
	fn new(path: PathBuf, name: FileName, metadata: Result<&Metadata, &std::io::Error>) -> Self {
		Self {
			path: Some(path.canonicalize().unwrap_or(path)),
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
	
	pub fn path(&self) -> Option<&Path> {
		self.path.as_deref()
	}
	
	pub fn name(&self) -> &FileName {
		&self.name
	}
	
	pub fn kind(&self) -> &FileKind {
		&self.kind
	}
	
	pub fn mode(&self) -> &FileMode {
		&self.mode
	}
	
	pub fn uid(&self) -> Option<u32> {
		self.owner.as_ref().map(FileOwner::uid)
	}
	
	pub fn gid(&self) -> Option<u32> {
		self.owner.as_ref().map(FileOwner::gid)
	}
	
	pub fn modified_time(&self) -> Option<&SystemTime> {
		self.mtime.as_ref()
	}
}

impl From<&DirEntry> for FileEntry {
	fn from(entry: &DirEntry) -> Self {
		let path = entry.path();
		let path = path.canonicalize().unwrap_or(path);
		let name = FileName::from(entry.file_name());
		return Self::new(path, name, entry.metadata().as_ref());
	}
}

impl From<&Path> for FileEntry {
	fn from(path: &Path) -> Self {
		let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
		let name = if path == Path::new("/") {
			FileName::from("/")
		} else {
			path.file_name().map(OsStr::to_os_string).map(FileName::from).unwrap_or_else(FileName::dummy)
		};
		
		let metadata = path.metadata();
		return Self::new(path, name, metadata.as_ref());
	}
}
