use slab_tree::{NodeId, NodeMut};

use crate::component::filesystem::tree::{FsTreeModel, FsTreeView, FsTreeViewNode};

impl FsTreeView {
	pub fn traverse_up_root(&mut self, model: &mut FsTreeModel) -> Option<NodeId> {
		let old_model_root_id = model.root_id();
		
		if let Some(new_model_root_id) = model.traverse_up_root() {
			self.set_root(new_model_root_id);
			
			if let Some(mut new_view_root) = self.get_mut(self.root_id) {
				Self::resolve_new_root_children(&mut new_view_root, model, old_model_root_id, new_model_root_id);
				Some(self.root_id)
			} else {
				None
			}
		} else {
			None
		}
	}
	
	fn resolve_new_root_children(new_view_root: &mut NodeMut<FsTreeViewNode>, model: &mut FsTreeModel, old_model_root_id: NodeId, new_model_root_id: NodeId) {
		new_view_root.data().is_expanded = true;
		
		if let Some(mut new_model_children) = model.resolve_children(new_model_root_id) {
			Self::sort_children(&mut new_model_children, model);
			
			for model_child_id in new_model_children {
				if model_child_id != old_model_root_id {
					new_view_root.append(FsTreeViewNode::from_model_node_id(model_child_id));
				} else if let Some(mut old_view_root) = new_view_root.first_child() {
					old_view_root.make_last_sibling();
				}
			}
		}
	}
}
