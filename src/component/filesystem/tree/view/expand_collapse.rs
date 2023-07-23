use slab_tree::{NodeId, NodeMut, RemoveBehavior};

use crate::component::filesystem::tree::{FsTreeModel, FsTreeView, FsTreeViewNode};

impl FsTreeView {
	pub fn expand(&mut self, view_node_id: NodeId, model: &mut FsTreeModel) -> bool {
		self.get_mut(view_node_id).map(|mut node| expand(&mut node, model)).unwrap_or(false)
	}
	
	pub fn collapse(&mut self, view_node_id: NodeId) -> bool {
		self.get_mut(view_node_id).map(|mut node| collapse(&mut node)).unwrap_or(false)
	}
	
	pub fn expand_or_collapse(&mut self, view_node_id: NodeId, model: &mut FsTreeModel) -> bool {
		self.get_mut(view_node_id).map(|mut node| expand_or_collapse(&mut node, model)).unwrap_or(false)
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

fn expand_or_collapse(node: &mut NodeMut<FsTreeViewNode>, model: &mut FsTreeModel) -> bool {
	if node.data().is_expanded() {
		collapse(node)
	} else {
		expand(node, model)
	}
}
