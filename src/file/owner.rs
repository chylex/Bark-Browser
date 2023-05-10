use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FileOwner {
	uid: u32,
	gid: u32,
}

impl From<&Metadata> for FileOwner {
	fn from(metadata: &Metadata) -> Self {
		Self {
			uid: metadata.uid(),
			gid: metadata.gid(),
		}
	}
}
