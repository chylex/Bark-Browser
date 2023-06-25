use std::collections::{HashMap, HashSet};

use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::tree::{FsTreeModel, FsTreeView, FsTreeViewNode};

impl FsTreeView {
	pub fn refresh_children(&mut self, parent_node_id: NodeId, model: &FsTreeModel) -> bool {
		if let Some(parent_node) = self.get(parent_node_id) {
			let parent_data = parent_node.data();
			
			if parent_data.is_expanded {
				let old_children = collect_old_model_to_view_node_id_map(&parent_node);
				let new_model_ids = collect_new_model_ids(model, parent_data);
				
				for new_model_id in &new_model_ids {
					if let Some(mut child_node) = old_children.get(new_model_id).and_then(|id| self.get_mut(*id)) {
						child_node.make_last_sibling();
					}
					else if let Some(mut parent_node) = self.get_mut(parent_node_id) {
						parent_node.append(FsTreeViewNode::from_model_node_id(*new_model_id));
					}
				}
				
				let new_model_node_id_set = HashSet::<_>::from_iter(new_model_ids);
				
				for (old_model_node_id, old_view_node_id) in old_children {
					if new_model_node_id_set.contains(&old_model_node_id) {
						self.refresh_children(old_view_node_id, model);
					} else {
						self.remove(old_view_node_id);
					}
				}
				
				return true
			}
		}
		
		false
	}
}

fn collect_old_model_to_view_node_id_map(parent_node: &NodeRef<FsTreeViewNode>) -> HashMap<NodeId, NodeId> {
	let mut old_children = HashMap::new();
	
	for old_child in parent_node.children() {
		old_children.insert(old_child.data().model_node_id, old_child.node_id());
	}
	
	old_children
}

fn collect_new_model_ids(model: &FsTreeModel, parent_data: &FsTreeViewNode) -> Vec<NodeId> {
	let mut new_model_children = model.get_children(parent_data.model_node_id).unwrap_or_default();
	
	FsTreeView::sort_children(&mut new_model_children, model);
	
	new_model_children
}
