use std::io;
use std::path::Path;

use slab_tree::NodeRef;

use crate::component::filesystem::FsLayer;
use crate::component::filesystem::tree::FsTreeViewNode;
use crate::file::FileEntry;

pub use self::create::*;
pub use self::delete::*;
pub use self::edit::*;
pub use self::rename::*;

mod create;
mod delete;
mod edit;
mod rename;

fn get_selected_file(layer: &FsLayer) -> Option<FileNode> {
	if let Some(node) = layer.selected_node() {
		if let Some(entry) = layer.tree.get_entry(&node) {
			if let Some(path) = entry.path() {
				return Some(FileNode { node, entry, path });
			}
		}
	}
	
	None
}

struct FileNode<'a> {
	node: NodeRef<'a, FsTreeViewNode>,
	entry: &'a FileEntry,
	path: &'a Path,
}

fn format_io_error(err: &io::Error) -> String {
	let mut str = if let Some(code) = err.raw_os_error() {
		err.to_string().replace(&format!(" (os error {code})"), "")
	} else {
		err.to_string()
	};
	
	str.push('.');
	str
}
