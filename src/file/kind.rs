use std::fs::Metadata;
use std::os::unix::fs::FileTypeExt;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FileKind {
	File { size: u64 },
	Directory,
	Symlink,
	BlockDevice,
	CharDevice,
	Pipe,
	Socket,
	Unknown,
}

impl From<&Metadata> for FileKind {
	fn from(metadata: &Metadata) -> Self {
		let file_type = metadata.file_type();
		
		if file_type.is_file() {
			Self::File { size: metadata.len() }
		} else if file_type.is_dir() {
			Self::Directory
		} else if file_type.is_symlink() {
			Self::Symlink
		} else if file_type.is_block_device() {
			Self::BlockDevice
		} else if file_type.is_char_device() {
			Self::CharDevice
		} else if file_type.is_fifo() {
			Self::Pipe
		} else if file_type.is_socket() {
			Self::Socket
		} else {
			Self::Unknown
		}
	}
}
