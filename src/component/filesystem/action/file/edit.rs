use std::{env, str};
use std::ffi::OsString;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

use ratatui::style::Color;
use slab_tree::NodeRef;

use crate::component::dialog::message::{MessageDialogActionMap, MessageDialogLayer};
use crate::component::filesystem::event::FsLayerEvent;
use crate::component::filesystem::FsLayer;
use crate::component::filesystem::tree::FsTreeViewNode;
use crate::file::FileEntry;
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;
use crate::util::slab_tree::NodeRefExtensions;

pub struct EditSelected;

impl Action<FsLayer> for EditSelected {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		if let Some(node) = layer.selected_node() {
			if let Some(entry_path) = layer.tree.get_entry(&node).and_then(FileEntry::path) {
				return edit_impl(layer, &node, entry_path);
			}
		}
		
		ActionResult::Nothing
	}
}

fn edit_impl(layer: &FsLayer, node: &NodeRef<FsTreeViewNode>, path: &Path) -> ActionResult {
	let editor = get_editor();
	let status = Command::new(&editor)
		.arg(path)
		.status();
	
	if status.is_err_and(|e| e.kind() == ErrorKind::NotFound) {
		return ActionResult::PushLayer(Box::new(MessageDialogLayer::new(Color::LightRed, "Error", format!("Default editor '{}' not found.", editor.to_string_lossy()), MessageDialogActionMap::ok())));
	}
	
	if let Some(parent_node_id) = node.parent_id() {
		FsLayerEvent::RefreshViewNodeChildren(parent_node_id).enqueue(&layer.pending_events);
	}
	
	ActionResult::Redraw
}

const DEFAULT_EDITOR: &str = "vim";

fn get_editor() -> OsString {
	env::var_os("VISUAL")
		.or_else(|| env::var_os("EDITOR"))
		.unwrap_or_else(|| OsString::from(DEFAULT_EDITOR))
}
