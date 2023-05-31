use slab_tree::{NodeId, NodeMut, RemoveBehavior};

use crate::component::filesystem::tree::{FsTreeModel, FsTreeView, FsTreeViewNode};

impl FsTreeView {
	pub fn expand(&mut self, view_node_id: NodeId, model: &mut FsTreeModel) -> bool {
		return if let Some(mut node) = self.get_mut(view_node_id) {
			expand(&mut node, model)
		} else {
			false
		};
	}
	
	pub fn collapse(&mut self, view_node_id: NodeId) -> bool {
		return if let Some(mut node) = self.get_mut(view_node_id) {
			collapse(&mut node)
		} else {
			false
		};
	}
	
	pub fn expand_or_collapse(&mut self, view_node_id: NodeId, model: &mut FsTreeModel) -> bool {
		if let Some(mut node) = self.get_mut(view_node_id) {
			if node.data().is_expanded() {
				collapse(&mut node)
			} else {
				expand(&mut node, model)
			}
		} else {
			false
		}
	}
}

pub fn expand(node: &mut NodeMut<FsTreeViewNode>, model: &mut FsTreeModel) -> bool {
	let data = node.data();
	if data.is_expanded {
		return false;
	}
	
	if let Some(mut children) = model.resolve_children(data.model_node_id) {
		data.is_expanded = true;
		
		FsTreeView::sort_children(&mut children, model);
		
		for child in children {
			node.append(FsTreeViewNode::from_model_node_id(child));
		}
		
		true
	} else {
		false
	}
}

pub fn collapse(node: &mut NodeMut<FsTreeViewNode>) -> bool {
	let data = node.data();
	if !data.is_expanded {
		return false;
	}
	
	data.is_expanded = false;
	while node.remove_first(RemoveBehavior::DropChildren).is_some() {}
	
	true
}
