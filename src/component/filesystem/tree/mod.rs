use std::path::Path;

use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::tree::view::FsTreeViewIterator;
use crate::file::FileEntry;

pub use self::model::FsTreeModel;
pub use self::model::FsTreeModelNode;
pub use self::view::FsTreeView;
pub use self::view::FsTreeViewNode;

mod model;
mod view;

pub struct FsTree {
	model: FsTreeModel,
	view: FsTreeView,
	pub selected_view_node_id: NodeId,
	structure_version: u32,
}

impl FsTree {
	pub fn with_root_path(path: &Path) -> Self {
		let model = FsTreeModel::with_root_path(path);
		let view = FsTreeView::from_model_root(&model);
		let root_id = view.root_id();
		
		let mut tree = Self {
			model,
			view,
			selected_view_node_id: root_id,
			structure_version: 0,
		};
		
		tree.expand(root_id);
		tree
	}
	
	pub const fn structure_version(&self) -> u32 {
		self.structure_version
	}
	
	pub fn selected_node(&self) -> Option<NodeRef<FsTreeViewNode>> {
		return self.view.get(self.selected_view_node_id);
	}
	
	pub fn view_root_node(&self) -> Option<NodeRef<FsTreeViewNode>> {
		self.view.root()
	}
	
	pub fn view_iter(&self) -> FsTreeViewIterator {
		self.view.into_iter()
	}
	
	pub fn get_view_node(&self, view_node_id: NodeId) -> Option<NodeRef<FsTreeViewNode>> {
		self.view.get(view_node_id)
	}
	
	pub fn get_entry(&self, node: &NodeRef<FsTreeViewNode>) -> Option<&FileEntry> {
		self.model
		    .get(node.data().model_node_id())
		    .map(|node| &node.data().entry)
	}
	
	pub fn expand(&mut self, view_node_id: NodeId) -> bool {
		let result = self.view.expand(view_node_id, &mut self.model);
		self.structure_changed_if_true(result)
	}
	
	pub fn collapse(&mut self, view_node_id: NodeId) -> bool {
		let result = self.view.collapse(view_node_id);
		self.structure_changed_if_true(result)
	}
	
	pub fn expand_or_collapse(&mut self, view_node_id: NodeId) -> bool {
		let result = self.view.expand_or_collapse(view_node_id, &mut self.model);
		self.structure_changed_if_true(result)
	}
	
	pub fn traverse_up_root(&mut self) -> Option<NodeId> {
		let new_root_id = self.view.traverse_up_root(&mut self.model);
		self.structure_changed_if(new_root_id, Option::is_some)
	}
	
	pub fn refresh_children(&mut self, view_node_id: NodeId) -> bool {
		if let Some(view_node) = self.view.get(view_node_id) {
			let result = self.model.refresh_children(view_node.data().model_node_id()) && self.view.refresh_children(view_node_id, &self.model);
			if result && self.selected_node().is_none() {
				self.selected_view_node_id = view_node_id;
			}
			self.structure_changed_if_true(result)
		} else {
			false
		}
	}
	
	pub fn select_child_node_by_name(&mut self, parent_view_node_id: NodeId, child_file_name: &str) -> bool {
		self.expand(parent_view_node_id);
		
		if let Some(parent_node) = self.view.get(parent_view_node_id) {
			for child_node in parent_node.children() {
				if self.get_entry(&child_node).is_some_and(|entry| entry.name().str() == child_file_name) {
					self.selected_view_node_id = child_node.node_id();
					return true;
				}
			}
		}
		
		false
	}
	
	pub fn delete_node(&mut self, view_node_id: NodeId) -> bool {
		let view = &mut self.view;
		
		if self.selected_view_node_id == view_node_id {
			self.selected_view_node_id = view.get(view_node_id).and_then(|node| node.below_id().or_else(|| node.above_id())).unwrap_or_else(|| view.root_id());
		}
		
		if let Some(view_node) = view.remove(view_node_id) {
			self.model.remove(view_node.model_node_id());
			true
		} else {
			false
		}
	}
	
	fn structure_changed(&mut self) {
		self.structure_version = self.structure_version.wrapping_add(1);
	}
	
	fn structure_changed_if<T, F>(&mut self, result: T, predicate: F) -> T where F: FnOnce(&T) -> bool {
		if predicate(&result) {
			self.structure_changed();
		}
		result
	}
	
	fn structure_changed_if_true(&mut self, result: bool) -> bool {
		self.structure_changed_if(result, |result| *result)
	}
}
