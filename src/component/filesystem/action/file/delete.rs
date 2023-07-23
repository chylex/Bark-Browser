use std::path::Path;
use std::rc::Rc;

use ratatui::style::Color;
use slab_tree::NodeId;

use crate::component::dialog::message::{MessageDialogActionMap, MessageDialogLayer};
use crate::component::filesystem::{FsLayer, FsLayerPendingEvents};
use crate::component::filesystem::event::FsLayerEvent;
use crate::file::FileEntry;
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;

pub struct DeleteSelected;

impl Action<FsLayer> for DeleteSelected {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		if let Some(view_node_to_delete) = layer.selected_node() {
			if let Some(entry_to_delete) = layer.tree.get_entry(&view_node_to_delete) {
				if let Some(dialog) = create_confirmation_dialog(entry_to_delete, Rc::clone(&layer.pending_events), view_node_to_delete.node_id()) {
					return ActionResult::PushLayer(Box::new(dialog));
				}
			}
		}
		
		ActionResult::Nothing
	}
}

fn create_confirmation_dialog<'a>(entry_to_delete: &FileEntry, pending_events: FsLayerPendingEvents, view_node_id_to_delete: NodeId) -> Option<MessageDialogLayer<'a>> {
	entry_to_delete.path().map(Path::to_path_buf).map(move |path| {
		MessageDialogLayer::new(Color::LightRed, "Confirm Deletion", format!("Delete {}?", path.to_string_lossy()), MessageDialogActionMap::yes_no(Box::new(move || {
			match delete_path_recursively(&path) {
				Ok(_) => {
					FsLayerEvent::DeleteViewNode(view_node_id_to_delete).enqueue(&pending_events);
					ActionResult::PopLayer
				}
				Err(e) => {
					ActionResult::ReplaceLayer(Box::new(MessageDialogLayer::new(Color::LightRed, "Error", e.to_string(), MessageDialogActionMap::ok())))
				}
			}
		})))
	})
}

fn delete_path_recursively<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
	if path.as_ref().is_dir() {
		std::fs::remove_dir_all(path)
	} else {
		std::fs::remove_file(path)
	}
}
