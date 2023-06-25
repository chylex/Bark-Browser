use std::path::Path;

use slab_tree::{NodeId, NodeRef};

use crate::file::FileEntry;

pub use self::model::FsTreeModel;
pub use self::model::FsTreeModelNode;
pub use self::view::FsTreeView;
pub use self::view::FsTreeViewNode;

mod model;
mod view;

pub struct FsTree {
	pub model: FsTreeModel,
	pub view: FsTreeView,
}

impl FsTree {
	pub fn with_root_path(path: &Path) -> Self {
		let model = FsTreeModel::with_root_path(path);
		let view = FsTreeView::from_model_root(&model);
		
		Self { model, view }
	}
	
	pub fn get_entry(&self, node: &NodeRef<FsTreeViewNode>) -> Option<&FileEntry> {
		self.model
		    .get(node.data().model_node_id())
		    .map(|node| &node.data().entry)
	}
	
	pub fn expand(&mut self, view_node_id: NodeId) -> bool {
		self.view.expand(view_node_id, &mut self.model)
	}
	
	pub fn collapse(&mut self, view_node_id: NodeId) -> bool {
		self.view.collapse(view_node_id)
	}
	
	pub fn expand_or_collapse(&mut self, view_node_id: NodeId) -> bool {
		self.view.expand_or_collapse(view_node_id, &mut self.model)
	}
	
	pub fn traverse_up_root(&mut self) -> Option<NodeId> {
		self.view.traverse_up_root(&mut self.model)
	}
	
	pub fn refresh_children(&mut self, view_node_id: NodeId) -> bool {
		if let Some(view_node) = self.view.get(view_node_id) {
			self.model.refresh_children(view_node.data().model_node_id()) && self.view.refresh_children(view_node_id, &self.model)
		} else {
			false
		}
	}
}
