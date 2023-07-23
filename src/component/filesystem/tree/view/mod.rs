use slab_tree::{NodeId, NodeMut, NodeRef, RemoveBehavior, Tree};

use crate::component::filesystem::tree::FsTreeModel;

mod above_below;
mod expand_collapse;
mod iterator;
mod refresh;
mod set_root;

pub struct FsTreeView {
	inner: Tree<FsTreeViewNode>,
	root_id: NodeId,
}

impl FsTreeView {
	pub fn from_model_root(model: &FsTreeModel) -> Self {
		let mut inner = Tree::new();
		let root_id = inner.set_root(FsTreeViewNode::from_model_node_id(model.root_id()));
		
		Self { inner, root_id }
	}
	
	pub const fn root_id(&self) -> NodeId {
		self.root_id
	}
	
	pub fn root(&self) -> Option<NodeRef<FsTreeViewNode>> {
		self.inner.root()
	}
	
	fn set_root(&mut self, model_id: NodeId) {
		self.root_id = self.inner.set_root(FsTreeViewNode::from_model_node_id(model_id));
	}
	
	pub fn get(&self, node_id: NodeId) -> Option<NodeRef<FsTreeViewNode>> {
		self.inner.get(node_id)
	}
	
	pub fn get_mut(&mut self, node_id: NodeId) -> Option<NodeMut<FsTreeViewNode>> {
		self.inner.get_mut(node_id)
	}
	
	pub fn remove(&mut self, node_id: NodeId) -> Option<FsTreeViewNode> {
		self.inner.remove(node_id, RemoveBehavior::DropChildren)
	}
	
	fn sort_children(children: &mut [NodeId], model: &FsTreeModel) {
		children.sort_by_key(|id| model.get(*id).map(|node| node.data().entry.name().str()));
	}
}

pub struct FsTreeViewNode {
	model_node_id: NodeId,
	is_expanded: bool,
}

impl FsTreeViewNode {
	pub const fn from_model_node_id(model_node_id: NodeId) -> Self {
		Self { model_node_id, is_expanded: false }
	}
	
	pub const fn model_node_id(&self) -> NodeId {
		self.model_node_id
	}
	
	pub const fn is_expanded(&self) -> bool {
		self.is_expanded
	}
}
