use std::collections::HashMap;
use std::path::Path;

use slab_tree::{NodeId, NodeRef};

use crate::component::filesystem::tree::{FsTreeModel, FsTreeModelNode};
use crate::file::FileEntry;

impl FsTreeModel {
	pub fn refresh_children(&mut self, parent_node_id: NodeId) -> bool {
		if let Some(parent_node) = self.get(parent_node_id).filter(|node| node.data().are_children_known) {
			let old_children = collect_old_nodes(&parent_node);
			let mut remaining_new_entries = collect_new_entries_as_optionals(&parent_node);
			
			let (update_node_ids, remove_node_ids) = compare_nodes(old_children, &remaining_new_entries);
			
			for remove_node_id in remove_node_ids {
				self.remove(remove_node_id);
			}
			
			for (update_node_id, new_entry_index) in &update_node_ids {
				if let Some(mut node) = self.get_mut(*update_node_id) {
					if let Some(new_entry) = remaining_new_entries.get_mut(*new_entry_index).and_then(Option::take) {
						node.data().entry = new_entry;
					} else {
						self.remove(*update_node_id);
					}
				}
			}
			
			if let Some(mut parent_node) = self.get_mut(parent_node_id) {
				for new_entry in remaining_new_entries.into_iter().flatten() {
					parent_node.append(FsTreeModelNode::from(new_entry));
				}
			}
			
			for (update_node_id, _) in update_node_ids {
				self.refresh_children(update_node_id);
			}
			
			true
		} else {
			false
		}
	}
}

fn collect_old_nodes<'a>(parent_node: &'a NodeRef<FsTreeModelNode>) -> Vec<(NodeId, &'a FileEntry)> {
	parent_node
		.children()
		.map(|child| (child.node_id(), &child.data().entry))
		.collect::<Vec<_>>()
}

fn collect_new_entries_as_optionals(parent_node: &NodeRef<FsTreeModelNode>) -> Vec<Option<FileEntry>> {
	let parent_entry = &parent_node.data().entry;
	
	FsTreeModel::get_directory_children(parent_entry)
		.map(|children| children.into_iter().map(Some).collect::<Vec<_>>())
		.unwrap_or_default()
}

fn create_file_entry_index(entries: &[Option<FileEntry>]) -> HashMap<&Path, usize> {
	let mut map = HashMap::new();
	
	for (i, entry) in entries.iter().enumerate() {
		if let Some(path) = entry.as_ref().and_then(FileEntry::path) {
			map.insert(path, i);
		}
	}
	
	map
}

fn compare_nodes(old_entries: Vec<(NodeId, &FileEntry)>, new_entries: &[Option<FileEntry>]) -> (Vec<(NodeId, usize)>, Vec<NodeId>) {
	let new_entry_index = create_file_entry_index(new_entries);
	
	let mut update_node_ids = vec![];
	let mut remove_node_ids = vec![];
	
	for (old_node_id, old_entry) in old_entries {
		if let Some(new_entry_index) = old_entry.path().and_then(|path| new_entry_index.get(path)) {
			update_node_ids.push((old_node_id, *new_entry_index));
		} else {
			remove_node_ids.push(old_node_id);
		}
	}
	
	(update_node_ids, remove_node_ids)
}
