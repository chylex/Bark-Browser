use std::fs::Metadata;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FileOwner {
	uid: u32,
	gid: u32,
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
