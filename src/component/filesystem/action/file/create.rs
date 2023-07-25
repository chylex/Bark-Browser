use std::{fs, io};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use ratatui::style::Color;
use slab_tree::NodeId;

use crate::component::dialog::message::{MessageDialogActionMap, MessageDialogLayer};
use crate::component::filesystem::event::{FsLayerEvent, FsLayerPendingEvents};
use crate::component::filesystem::FsLayer;
use crate::component::input::InputFieldLayer;
use crate::file::FileKind;
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;
use crate::util::slab_tree::NodeRefExtensions;

pub struct CreateFile;

impl Action<FsLayer> for CreateFile {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		create_impl(layer, "file", |p| fs::write(p, b""))
	}
}

pub struct CreateDirectory;

impl Action<FsLayer> for CreateDirectory {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		create_impl(layer, "directory", fs::create_dir)
	}
}

fn create_impl<F>(layer: &mut FsLayer, kind: &'static str, create_function: F) -> ActionResult where F: Fn(PathBuf) -> io::Result<()> + 'static {
	if let Some(selected_node) = layer.selected_node() {
		if let Some(selected_node_entry) = layer.tree.get_entry(&selected_node) {
			let parent_folder_path;
			let parent_node_id;
			
			if matches!(selected_node_entry.kind(), FileKind::Directory) {
				parent_folder_path = selected_node_entry.path();
				parent_node_id = Some(selected_node.node_id());
			} else {
				parent_folder_path = selected_node_entry.path().and_then(Path::parent);
				parent_node_id = selected_node.parent_id();
			};
			
			if let Some(parent_folder_path) = parent_folder_path {
				if let Some(parent_node_id) = parent_node_id {
					return ActionResult::PushLayer(Box::new(create_prompt(kind, parent_folder_path.to_path_buf(), create_function, Rc::clone(&layer.pending_events), parent_node_id)));
				}
			}
		}
	}
	
	ActionResult::Nothing
}

fn create_prompt<F>(kind: &'static str, parent_folder: PathBuf, create_function: F, pending_events: FsLayerPendingEvents, view_node_id_to_refresh: NodeId) -> InputFieldLayer where F: Fn(PathBuf) -> io::Result<()> + 'static {
	InputFieldLayer::new(Box::new(move |file_name| {
		if file_name.is_empty() {
			return ActionResult::Nothing;
		}
		
		let file_path = parent_folder.join(file_name);
		
		if file_path.exists() {
			let file_path = file_path.to_string_lossy();
			return ActionResult::PushLayer(Box::new(MessageDialogLayer::new(Color::LightRed, "Error", format!("File or directory {file_path} already exists."), MessageDialogActionMap::ok())));
		}
		
		match create_function(file_path) {
			Ok(_) => {
				FsLayerEvent::RefreshViewNodeChildren(view_node_id_to_refresh).enqueue(&pending_events);
				ActionResult::PopLayer
			}
			Err(e) => {
				ActionResult::PushLayer(Box::new(MessageDialogLayer::new(Color::LightRed, "Error", format!("Could not create {kind}: {e}"), MessageDialogActionMap::ok())))
			}
		}
	}))
}
