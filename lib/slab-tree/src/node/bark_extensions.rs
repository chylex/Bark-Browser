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
