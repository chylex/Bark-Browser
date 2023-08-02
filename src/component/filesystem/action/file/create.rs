use std::{fs, io};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use ratatui::style::Color;
use slab_tree::NodeId;

use crate::component::dialog::input::InputFieldDialogLayer;
use crate::component::dialog::message::MessageDialogLayer;
use crate::component::filesystem::action::file::{FileNode, get_selected_file};
use crate::component::filesystem::event::FsLayerEvent;
use crate::component::filesystem::FsLayer;
use crate::file::FileKind;
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;
use crate::util::slab_tree::NodeRefExtensions;

trait CreateEntry {
	fn title() -> &'static str;
	fn kind() -> &'static str;
	fn create(path: PathBuf) -> io::Result<()>;
}

pub struct CreateFile;

impl CreateEntry for CreateFile {
	fn title() -> &'static str {
		"Create File"
	}
	
	fn kind() -> &'static str {
		"file"
	}
	
	fn create(path: PathBuf) -> io::Result<()> {
		fs::write(path, b"")
	}
}

pub struct CreateDirectory;

impl CreateEntry for CreateDirectory {
	fn title() -> &'static str {
		"Create Directory"
	}
	
	fn kind() -> &'static str {
		"directory"
	}
	
	fn create(path: PathBuf) -> io::Result<()> {
		fs::create_dir(path)
	}
}

pub struct CreateFileInSelectedDirectory;

impl Action<FsLayer> for CreateFileInSelectedDirectory {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		create_in_selected_directory::<CreateFile>(layer)
	}
}

pub struct CreateDirectoryInSelectedDirectory;

impl Action<FsLayer> for CreateDirectoryInSelectedDirectory {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		create_in_selected_directory::<CreateDirectory>(layer)
	}
}

fn create_in_selected_directory<T: CreateEntry>(layer: &mut FsLayer) -> ActionResult {
	if let Some(FileNode { node, path, .. }) = get_selected_file(layer).filter(|n| matches!(n.entry.kind(), FileKind::Directory)) {
		ActionResult::PushLayer(Box::new(create_new_name_prompt::<T>(layer, path.to_owned(), node.node_id())))
	} else {
		ActionResult::Nothing
	}
}

pub struct CreateFileInParentOfSelectedEntry;

impl Action<FsLayer> for CreateFileInParentOfSelectedEntry {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		create_in_parent_of_selected_file::<CreateFile>(layer)
	}
}

pub struct CreateDirectoryInParentOfSelectedEntry;

impl Action<FsLayer> for CreateDirectoryInParentOfSelectedEntry {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		create_in_parent_of_selected_file::<CreateDirectory>(layer)
	}
}

fn create_in_parent_of_selected_file<T: CreateEntry>(layer: &mut FsLayer) -> ActionResult {
	if let Some((parent_node_id, parent_path)) = get_parent_of_selected_file(layer) {
		ActionResult::PushLayer(Box::new(create_new_name_prompt::<T>(layer, parent_path.to_owned(), parent_node_id)))
	} else {
		ActionResult::Nothing
	}
}

fn get_parent_of_selected_file(layer: &FsLayer) -> Option<(NodeId, &Path)> {
	get_selected_file(layer).and_then(|n| { Some((n.node.parent_id()?, n.path.parent()?)) })
}

fn create_new_name_prompt<'b, T: CreateEntry>(layer: &FsLayer, parent_folder: PathBuf, view_node_id_to_refresh: NodeId) -> InputFieldDialogLayer<'b> {
	let y = layer.dialog_y();
	let pending_events = Rc::clone(&layer.pending_events);
	
	InputFieldDialogLayer::build()
		.y(y)
		.min_width(40)
		.color(Color::LightCyan, Color::Cyan)
		.title(T::title())
		.message(format!("Creating {} in {}", T::kind(), parent_folder.to_string_lossy()))
		.build(Box::new(move |new_name| {
			if new_name.is_empty() {
				return ActionResult::Nothing;
			}
			
			let new_path = parent_folder.join(&new_name);
			if new_path.exists() {
				return ActionResult::PushLayer(Box::new(MessageDialogLayer::generic_error(y, "Something with this name already exists.")));
			}
			
			match T::create(new_path) {
				Ok(_) => {
					FsLayerEvent::RefreshViewNodeChildren(view_node_id_to_refresh).enqueue(&pending_events);
					FsLayerEvent::SelectViewNodeChildByFileName(view_node_id_to_refresh, new_name).enqueue(&pending_events);
					ActionResult::PopLayer
				}
				Err(e) => {
					ActionResult::PushLayer(Box::new(MessageDialogLayer::generic_error(y, format!("Could not create {}: {e}", T::kind()))))
				}
			}
		}))
}
