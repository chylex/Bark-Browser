use io::ErrorKind;
use std::{fs, io};
use std::path::PathBuf;
use std::rc::Rc;

use ratatui::style::Color;
use slab_tree::NodeRef;

use crate::component::dialog::input::InputFieldDialogLayer;
use crate::component::dialog::message::MessageDialogLayer;
use crate::component::filesystem::action::file::{FileNode, format_io_error, get_entry_kind_name, get_selected_file};
use crate::component::filesystem::event::FsLayerEvent;
use crate::component::filesystem::FsLayer;
use crate::component::filesystem::tree::FsTreeViewNode;
use crate::file::FileEntry;
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;
use crate::util::slab_tree::NodeRefExtensions;

pub struct RenameSelectedEntry {
	pub prefill: bool,
}

impl Action<FsLayer> for RenameSelectedEntry {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		if let Some(FileNode { node, entry, path }) = get_selected_file(layer) {
			ActionResult::PushLayer(Box::new(self.create_rename_dialog(layer, &node, entry, path.to_owned())))
		} else {
			ActionResult::Nothing
		}
	}
}

impl RenameSelectedEntry {
	#[allow(clippy::wildcard_enum_match_arm)]
	fn create_rename_dialog<'a, 'b>(&'a self, layer: &'a FsLayer, node: &'a NodeRef<FsTreeViewNode>, entry: &'a FileEntry, path: PathBuf) -> InputFieldDialogLayer<'b> {
		let y = layer.dialog_y();
		let parent_node_id = node.parent_id();
		let pending_events = Rc::clone(&layer.pending_events);
		
		InputFieldDialogLayer::build()
			.y(y)
			.min_width(40)
			.color(Color::LightCyan, Color::Cyan)
			.title(format!("Rename {}", get_entry_kind_name(entry)))
			.message(format!("Renaming {}", path.to_string_lossy()))
			.initial_value(self.prefill.then(|| entry.name().str().to_owned()))
			.build(Box::new(move |new_name| {
				match rename_file(&path, &new_name) {
					Ok(_) => {
						if let Some(parent_node_id) = parent_node_id {
							FsLayerEvent::RefreshViewNodeChildren(parent_node_id).enqueue(&pending_events);
							FsLayerEvent::SelectViewNodeChildByFileName(parent_node_id, new_name).enqueue(&pending_events);
						}
						ActionResult::PopLayer
					}
					Err(e) => {
						ActionResult::PushLayer(Box::new(MessageDialogLayer::generic_error(y.saturating_add(1), format_io_error(&e))))
					}
				}
			}))
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
