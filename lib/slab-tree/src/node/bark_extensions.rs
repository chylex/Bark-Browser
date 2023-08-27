use crate::NodeId;
use crate::NodeMut;
use crate::NodeRef;

impl<'a, T>  NodeRef<'a, T> {
	pub fn parent_id(&self) -> Option<NodeId> {
		self.parent().map(|node| node.node_id())
	}
	
	pub fn first_child_id(&self) -> Option<NodeId> {
		self.first_child().map(|node| node.node_id())
	}
	
	pub fn last_child_id(&self) -> Option<NodeId> {
		self.last_child().map(|node| node.node_id())
	}
	
	pub fn prev_sibling_id(&self) -> Option<NodeId> {
		self.prev_sibling().map(|node| node.node_id())
	}
	
	pub fn next_sibling_id(&self) -> Option<NodeId> {
		self.next_sibling().map(|node| node.node_id())
	}
	
	pub fn last_descendant_or_self(&self) -> NodeRef<'a, T> {
		let mut node = NodeRef::new(self.node_id(), self.tree);
		
		while let Some(id) = node.get_self_as_node().relatives.last_child {
			node = NodeRef::new(id, self.tree);
		}
		
		node
	}
	
	pub fn above(&self) -> Option<NodeRef<T>> {
		if let Some(prev_sibling) = self.prev_sibling() {
			Some(prev_sibling.last_descendant_or_self())
		} else {
			self.parent()
		}
	}
	
	pub fn above_id(&self) -> Option<NodeId> {
		self.above().map(|node| node.node_id())
	}
	
	pub fn below(&self) -> Option<NodeRef<T>> {
		self.first_child()
			.or_else(|| self.next_sibling())
			.or_else(|| self.ancestors().find_map(|ancestor| ancestor.next_sibling_id()).map(|id| NodeRef::new(id, self.tree)))
	}
	
	pub fn below_id(&self) -> Option<NodeId> {
		self.below().map(|node| node.node_id())
	}
}

impl<'a, T> NodeMut<'a, T> {
	pub fn parent_id(&mut self) -> Option<NodeId> {
		self.parent().map(|node| node.node_id())
	}
	
	pub fn first_child_id(&mut self) -> Option<NodeId> {
		self.first_child().map(|node| node.node_id())
	}
	
	pub fn last_child_id(&mut self) -> Option<NodeId> {
		self.last_child().map(|node| node.node_id())
	}
	
	pub fn prev_sibling_id(&mut self) -> Option<NodeId> {
		self.prev_sibling().map(|node| node.node_id())
	}
	
	pub fn next_sibling_id(&mut self) -> Option<NodeId> {
		self.next_sibling().map(|node| node.node_id())
	}
}
