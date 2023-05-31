use slab_tree::iter::PreOrder;
use slab_tree::NodeRef;

use crate::component::filesystem::tree::{FsTreeView, FsTreeViewNode};

impl<'a> IntoIterator for &'a FsTreeView {
	type Item = NodeRef<'a, FsTreeViewNode>;
	type IntoIter = FsTreeViewIterator<'a>;
	
	fn into_iter(self) -> Self::IntoIter {
		FsTreeViewIterator {
			iter: self.inner.root().map(|root| root.traverse_pre_order())
		}
	}
}

pub struct FsTreeViewIterator<'a> {
	iter: Option<PreOrder<'a, FsTreeViewNode>>,
}

impl<'a> Iterator for FsTreeViewIterator<'a> {
	type Item = NodeRef<'a, FsTreeViewNode>;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.iter.as_mut().and_then(PreOrder::next)
	}
}
