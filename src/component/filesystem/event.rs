use std::cell::RefCell;
use std::rc::Rc;

use slab_tree::NodeId;

use crate::component::filesystem::FsLayer;

pub type FsLayerPendingEvents = Rc<RefCell<Vec<FsLayerEvent>>>;

pub enum FsLayerEvent {
	RefreshViewNodeChildren(NodeId),
	SelectViewNodeChildByFileName(NodeId, String),
	DeleteViewNode(NodeId),
}

impl FsLayerEvent {
	pub fn enqueue(self, pending_events: &FsLayerPendingEvents) -> bool {
		if let Ok(mut pending_events) = pending_events.try_borrow_mut() {
			pending_events.push(self);
			true
		} else {
			false
		}
	}
	
	pub fn handle(self, layer: &mut FsLayer) {
		match self {
			Self::RefreshViewNodeChildren(view_node_id) => handle_refresh_view_node_children(layer, view_node_id),
			Self::SelectViewNodeChildByFileName(parent_view_node_id, child_file_name) => handle_select_view_node_child_by_name(layer, parent_view_node_id, child_file_name.as_str()),
			Self::DeleteViewNode(view_node_id) => handle_delete_view_node(layer, view_node_id),
		}
	}
}

fn handle_refresh_view_node_children(layer: &mut FsLayer, view_node_id: NodeId) {
	layer.refresh_children(view_node_id);
}

fn handle_select_view_node_child_by_name(layer: &mut FsLayer, parent_view_node_id: NodeId, child_file_name: &str) {
	layer.tree.expand(parent_view_node_id);
	
	if let Some(parent_node) = layer.tree.view.get(parent_view_node_id) {
		for child_node in parent_node.children() {
			if layer.tree.get_entry(&child_node).is_some_and(|entry| entry.name().str() == child_file_name) {
				layer.selected_view_node_id = child_node.node_id();
				return;
			}
		}
	}
}

fn handle_delete_view_node(layer: &mut FsLayer, view_node_id: NodeId) {
	let view = &mut layer.tree.view;
	
	if layer.selected_view_node_id == view_node_id {
		layer.selected_view_node_id = view.get_node_above_id(view_node_id).unwrap_or_else(|| view.root_id());
	}
	
	if let Some(view_node) = view.remove(view_node_id) {
		layer.tree.model.remove(view_node.model_node_id());
	}
}
