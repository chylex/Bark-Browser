use std::ffi::OsStr;
use std::fs::{DirEntry, Metadata};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub use crate::file::kind::FileKind;
pub use crate::file::mode::{FileMode, Permission, PermissionClassMode};
pub use crate::file::name::FileName;
pub use crate::file::owner::FileOwner;

mod kind;
mod mode;
mod owner;
mod name;

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
			mode: metadata.map(|m| FileMode::Known(m.mode())).unwrap_or(FileMode::Unknown),
			owner: metadata.map(FileOwner::from).ok(),
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
