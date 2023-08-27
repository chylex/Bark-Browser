use io::ErrorKind;
use std::{fs, io};
use std::path::PathBuf;

use ratatui::style::Color;

use slab_tree::NodeRef;

use crate::component::dialog::input::InputFieldDialogLayer;
use crate::component::dialog::message::MessageDialogLayer;
use crate::component::filesystem::action::file::{FileNode, format_io_error, get_entry_kind_name, get_selected_file, RefreshParentDirectoryAndSelectFile};
use crate::component::filesystem::FsLayer;
use crate::component::filesystem::tree::FsTreeViewNode;
use crate::file::FileEntry;
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;

pub struct RenameSelectedEntry {
	pub prefill: bool,
}

impl Action<FsLayer> for RenameSelectedEntry {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		if let Some(FileNode { node, entry, path }) = get_selected_file(layer) {
			ActionResult::push_layer(self.create_rename_dialog(layer, &node, entry, path.to_owned()))
		} else {
			ActionResult::Nothing
		}
	}
}

impl RenameSelectedEntry {
	#[allow(clippy::wildcard_enum_match_arm)]
	fn create_rename_dialog<'a, 'b>(&'a self, layer: &'a FsLayer, node: &'a NodeRef<FsTreeViewNode>, entry: &'a FileEntry, path: PathBuf) -> InputFieldDialogLayer<'b> {
		let y = layer.dialog_y();
		let events = layer.events();
		let parent_view_node_id = node.parent_id();
		
		InputFieldDialogLayer::build()
			.y(y)
			.min_width(40)
			.color(Color::LightCyan, Color::Cyan)
			.title(format!("Rename {}", get_entry_kind_name(entry)))
			.message(format!("Renaming {}", path.to_string_lossy()))
			.initial_value(self.prefill.then(|| entry.name().str().to_owned()))
			.on_confirm(move |new_name| {
				match rename_file(&path, &new_name) {
					Ok(_) => {
						if let Some(parent_view_node_id) = parent_view_node_id {
							events.enqueue(RefreshParentDirectoryAndSelectFile { parent_view_node_id, child_file_name: new_name });
						}
						ActionResult::PopLayer
					}
					Err(e) => {
						ActionResult::push_layer(MessageDialogLayer::error(y.saturating_add(1), format_io_error(&e)))
					}
				}
			})
	}
}

fn rename_file(path: &PathBuf, new_name: &String) -> io::Result<()> {
	let new_path = path.with_file_name(new_name);
	
	if new_path.components().count() != path.components().count() {
		Err(io::Error::new(ErrorKind::InvalidInput, "Invalid name"))
	} else if fs::symlink_metadata(&new_path).is_ok() {
		Err(io::Error::new(ErrorKind::AlreadyExists, "Something with this name already exists"))
	} else {
		fs::rename(path, new_path)
	}
}
