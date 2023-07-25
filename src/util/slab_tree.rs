use slab_tree::{NodeId, NodeMut, NodeRef};

pub trait NodeRefExtensions {
	fn parent_id(&self) -> Option<NodeId>;
	fn first_child_id(&self) -> Option<NodeId>;
	fn last_child_id(&self) -> Option<NodeId>;
	fn prev_sibling_id(&self) -> Option<NodeId>;
	fn next_sibling_id(&self) -> Option<NodeId>;
}

pub trait NodeMutExtensions {
	fn parent_id(&mut self) -> Option<NodeId>;
	fn first_child_id(&mut self) -> Option<NodeId>;
	fn last_child_id(&mut self) -> Option<NodeId>;
	fn prev_sibling_id(&mut self) -> Option<NodeId>;
	fn next_sibling_id(&mut self) -> Option<NodeId>;
}

impl<'a, T> NodeRefExtensions for NodeRef<'a, T> {
	fn parent_id(&self) -> Option<NodeId> {
		self.parent().map(|node| node.node_id())
	}
	
	fn first_child_id(&self) -> Option<NodeId> {
		self.first_child().map(|node| node.node_id())
	}
	
	fn last_child_id(&self) -> Option<NodeId> {
		self.last_child().map(|node| node.node_id())
	}
	
	fn prev_sibling_id(&self) -> Option<NodeId> {
		self.prev_sibling().map(|node| node.node_id())
	}
	
	fn next_sibling_id(&self) -> Option<NodeId> {
		self.next_sibling().map(|node| node.node_id())
	}
}

impl<'a, T> NodeMutExtensions for NodeMut<'a, T> {
	fn parent_id(&mut self) -> Option<NodeId> {
		self.parent().map(|node| node.node_id())
	}
	
	fn first_child_id(&mut self) -> Option<NodeId> {
		self.first_child().map(|node| node.node_id())
	}
	
	fn last_child_id(&mut self) -> Option<NodeId> {
		self.last_child().map(|node| node.node_id())
	}
	
	fn prev_sibling_id(&mut self) -> Option<NodeId> {
		self.prev_sibling().map(|node| node.node_id())
	}
	
	fn next_sibling_id(&mut self) -> Option<NodeId> {
		self.next_sibling().map(|node| node.node_id())
	}
}
