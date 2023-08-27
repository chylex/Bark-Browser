use std::path::Path;

use crate::component::filesystem::registers::FsTreeRegisters;
use crate::component::filesystem::tree::FsTree;
use crate::file::FileOwnerNameCache;
use crate::input::keymap::{KeyBinding, KeyMapLookupResult};
use crate::state::action::ActionResult;
use crate::state::Environment;
use crate::state::event::{EventQueue, EventResult};
use crate::state::layer::Layer;
use crate::state::view::Frame;

mod action;
mod render;
mod tree;
mod registers;

pub struct FsLayer {
	pub tree: FsTree,
	tree_structure_version: u32,
	pub registers: FsTreeRegisters,
	cursor_y: u16,
	pending_keys: Vec<KeyBinding>,
	event_queue: EventQueue<FsLayer>,
	file_owner_name_cache: FileOwnerNameCache,
	column_width_cache: Option<ColumnWidths>,
}

impl FsLayer {
	pub fn with_root_path(root_path: &Path) -> Self {
		// Initialize action map early in case it errors.
		let _ = *action::ACTION_MAP;
		
		Self {
			tree: FsTree::with_root_path(root_path),
			tree_structure_version: 0,
			cursor_y: 0,
			registers: FsTreeRegisters::new(),
			pending_keys: Vec::new(),
			event_queue: EventQueue::new(),
			file_owner_name_cache: FileOwnerNameCache::new(),
			column_width_cache: None,
		}
	}
	
	pub fn events(&self) -> EventQueue<Self> {
		self.event_queue.rc_clone()
	}
	
	pub const fn dialog_y(&self) -> u16 {
		self.cursor_y.saturating_add(1)
	}
}

impl Layer for FsLayer {
	fn handle_input(&mut self, environment: &Environment, key_binding: KeyBinding) -> ActionResult {
		self.pending_keys.push(key_binding);
		
		match action::ACTION_MAP.lookup(&self.pending_keys) {
			KeyMapLookupResult::Prefix => {
				ActionResult::Nothing
			}
			
			KeyMapLookupResult::Found(action) => {
				self.pending_keys.clear();
				
				let old_count = self.registers.count;
				let result = action.perform(self, environment);
				
				// Reset count after every action, unless the action modified it.
				if old_count == self.registers.count {
					self.registers.count = None;
				}
				
				result
			}
			
			KeyMapLookupResult::None => {
				self.pending_keys.clear();
				self.registers.count = None;
				ActionResult::Nothing
			}
		}
	}
	
	fn handle_events(&mut self, environment: &Environment) -> EventResult {
		self.event_queue.take().into_iter().fold(EventResult::Nothing, |result, event| result.merge(event.dispatch(self, environment)))
	}
	
	fn render(&mut self, frame: &mut Frame) {
		if self.tree_structure_version != self.tree.structure_version() {
			self.tree_structure_version = self.tree.structure_version();
			self.column_width_cache.take();
		}
		
		render::render(self, frame);
	}
}

#[derive(Copy, Clone, Default)]
pub struct ColumnWidths {
	pub name: u16,
	pub user: u16,
	pub group: u16,
}

impl ColumnWidths {
	const fn user_and_group(self) -> u16 {
		self.user.saturating_add(1).saturating_add(self.group)
	}
}
