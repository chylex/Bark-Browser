use std::cell::RefCell;
use std::rc::Rc;

use slab_tree::NodeId;

use crate::component::filesystem::FsLayer;

pub enum FsLayerEvent {
	DeleteViewNode(NodeId)
}

impl FsLayerEvent {
	pub fn push(pending_events: Rc<RefCell<Vec<FsLayerEvent>>>, event: FsLayerEvent) -> bool {
		if let Ok(mut pending_events) = pending_events.try_borrow_mut() {
			pending_events.push(event);
			true
		} else {
			false
		}
	}
	
	pub fn handle(&self, layer: &mut FsLayer) {
		match self {
			FsLayerEvent::DeleteViewNode(view_node_id) => handle_delete_view_node(layer, view_node_id),
		}
	}
}

fn handle_delete_view_node(layer: &mut FsLayer, view_node_id: &NodeId) {
	let view = &mut layer.tree.view;
	
	if layer.selected_view_node_id == *view_node_id {
		layer.selected_view_node_id = view.get_node_above_id(*view_node_id).unwrap_or_else(|| view.root_id());
	}
	
	if let Some(view_node) = view.remove(*view_node_id) {
		layer.tree.model.remove(view_node.model_node_id());
	}
}
