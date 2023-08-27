use std::io;
use std::path::Path;

use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::FsLayer;
use crate::component::filesystem::tree::FsTreeViewNode;
use crate::file::{FileEntry, FileKind};
use crate::state::Environment;
use crate::state::event::{Event, EventResult};

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

#[allow(clippy::wildcard_enum_match_arm)]
const fn get_entry_kind_name(entry: &FileEntry) -> &'static str {
	match entry.kind() {
		FileKind::Directory => "Directory",
		FileKind::Symlink => "Symbolic Link",
		_ => "File"
	}
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

struct RefreshParentDirectoryAndSelectFile {
	parent_view_node_id: NodeId,
	child_file_name: String,
}

impl Event<FsLayer> for RefreshParentDirectoryAndSelectFile {
	fn dispatch(&self, layer: &mut FsLayer, _environment: &Environment) -> EventResult {
		if layer.refresh_children(self.parent_view_node_id) {
			layer.select_child_node_by_name(self.parent_view_node_id, &self.child_file_name);
			EventResult::Draw
		} else {
			EventResult::Nothing
		}
	}
}
