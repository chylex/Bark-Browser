use std::time::{Duration, Instant};

use ratatui::style::Color;
use slab_tree::NodeId;

use crate::component::dialog::message::MessageDialogLayer;
use crate::component::filesystem::FsLayer;
use crate::component::filesystem::tree::FsTree;
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;

pub struct ExpandCollapse {
	pub default_depth: usize,
}

impl Action<FsLayer> for ExpandCollapse {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		let depth = layer.registers.count.unwrap_or(self.default_depth);
		if depth == 0 {
			return ActionResult::Nothing;
		}
		
		if layer.tree.expand_or_collapse(layer.tree.selected_view_node_id) {
			if depth > 1 {
				if let Some(node) = layer.tree.selected_node() {
					if node.data().is_expanded() {
						let child_node_ids = node.children().map(|node| node.node_id()).collect();
						let remaining_depth = depth.saturating_sub(1);
						if !expand_children_to_depth(&mut layer.tree, child_node_ids, remaining_depth) {
							return ActionResult::push_layer(MessageDialogLayer::build()
								.y(layer.dialog_y())
								.color(Color::LightYellow)
								.title("Expansion Stopped")
								.message(format!("Expansion was taking more than {} seconds, stopping now.", MAX_EXPANSION_TIME.as_secs()))
								.ok());
						}
					}
				}
			}
			
			ActionResult::Draw
		} else {
			ActionResult::Nothing
		}
	}
}

const MAX_EXPANSION_TIME: Duration = Duration::from_secs(10);

fn expand_children_to_depth(tree: &mut FsTree, mut child_node_ids: Vec<NodeId>, max_depth: usize) -> bool {
	let start_time = Instant::now();
	let mut current_pass_node_ids = Vec::new();
	
	for _depth in 0..max_depth {
		current_pass_node_ids.append(&mut child_node_ids);
		
		for node_id in &current_pass_node_ids {
			let node_id = *node_id;
			tree.expand(node_id);
			get_child_node_ids(tree, node_id, &mut child_node_ids);
			
			if start_time.elapsed() >= MAX_EXPANSION_TIME {
				return false;
			}
		}
	}
	
	true
}

fn get_child_node_ids(tree: &FsTree, node_id: NodeId, output_node_ids: &mut Vec<NodeId>) {
	if let Some(node) = tree.get_view_node(node_id) {
		for child in node.children() {
			output_node_ids.push(child.node_id());
		}
	}
}
