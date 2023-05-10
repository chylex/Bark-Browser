use std::ffi::{OsStr, OsString};
use std::fs::{DirEntry, Metadata};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub use crate::file::kind::FileKind;
pub use crate::file::mode::{FileMode, Permission, PermissionClassMode};
pub use crate::file::owner::FileOwner;

mod kind;
mod mode;
mod owner;

pub struct FileEntry {
	path: Option<PathBuf>,
	name: Option<OsString>,
	kind: FileKind,
	mode: FileMode,
	owner: Option<FileOwner>,
	mtime: Option<SystemTime>,
}

impl FileEntry {
	fn new(path: PathBuf, name: Option<OsString>, metadata: Result<&Metadata, &std::io::Error>) -> Self {
		let path = path.canonicalize().unwrap_or(path);
		let name = name.unwrap_or_else(|| path.as_os_str().to_os_string());
		
		Self {
			path: Some(path),
			name: Some(name),
			kind: metadata.map(FileKind::from).unwrap_or(FileKind::Unknown),
			mode: metadata.map(|m| FileMode::Known(m.mode())).unwrap_or(FileMode::Unknown),
			owner: metadata.map(FileOwner::from).ok(),
			mtime: metadata.ok().and_then(|m| m.modified().ok()),
		}
	}
	
	pub fn dummy() -> Self {
		Self {
			path: None,
			name: None,
			kind: FileKind::Unknown,
			mode: FileMode::Unknown,
			owner: None,
			mtime: None
		}
	}
	
	pub fn path(&self) -> Option<&Path> {
		return self.path.as_deref();
	}
	
	pub fn name(&self) -> Option<&OsStr> {
		return self.name.as_deref();
	}
	
	pub fn kind(&self) -> FileKind {
		self.kind
	}
	
	pub fn mode(&self) -> FileMode {
		self.mode
	}
}

impl From<&DirEntry> for FileEntry {
	fn from(entry: &DirEntry) -> Self {
		return Self::new(entry.path(), Some(entry.file_name()), entry.metadata().as_ref());
	}
}

impl From<&Path> for FileEntry {
	fn from(path: &Path) -> Self {
		return Self::new(path.to_path_buf(), path.file_name().map(|n| n.to_os_string()), path.metadata().as_ref());
	}
}
