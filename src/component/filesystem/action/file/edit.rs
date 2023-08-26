use std::{env, str};
use std::ffi::OsString;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

use slab_tree::NodeRef;

use crate::component::dialog::message::MessageDialogLayer;
use crate::component::filesystem::action::file::{FileNode, get_selected_file};
use crate::component::filesystem::event::FsLayerEvent;
use crate::component::filesystem::FsLayer;
use crate::component::filesystem::tree::FsTreeViewNode;
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;
use crate::util::slab_tree::NodeRefExtensions;

pub struct EditSelectedEntry;

impl Action<FsLayer> for EditSelectedEntry {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		if let Some(FileNode { node, path, .. }) = get_selected_file(layer) {
			open_default_editor(layer, &node, path)
		} else {
			ActionResult::Nothing
		}
	}
}

fn open_default_editor(layer: &FsLayer, node: &NodeRef<FsTreeViewNode>, path: &Path) -> ActionResult {
	let editor = get_editor();
	let status = Command::new(&editor)
		.arg(path)
		.status();
	
	if status.is_err_and(|e| e.kind() == ErrorKind::NotFound) {
		return ActionResult::push_layer(MessageDialogLayer::error(layer.dialog_y(), format!("Default editor '{}' not found.", editor.to_string_lossy())));
	}
	
	// Refresh the parent directory, or the root node if this is the view root.
	let node_id_to_refresh = node.parent_id().unwrap_or_else(|| node.node_id());
	FsLayerEvent::RefreshViewNodeChildren(node_id_to_refresh).enqueue(&layer.pending_events);
	
	ActionResult::Redraw
}

const DEFAULT_EDITOR: &str = "vim";

fn get_editor() -> OsString {
	env::var_os("VISUAL")
		.or_else(|| env::var_os("EDITOR"))
		.unwrap_or_else(|| OsString::from(DEFAULT_EDITOR))
}
