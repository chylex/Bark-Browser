use crossterm::event::{KeyCode, KeyModifiers};
use lazy_static::lazy_static;

use crate::component::filesystem::action::file::DeleteSelected;
use crate::component::filesystem::action::movement::{MoveDown, MoveOrTraverseUpParent, MoveToNextSibling, MoveToPreviousSibling, MoveUp};
use crate::component::filesystem::action::quit::Quit;
use crate::component::filesystem::action::tree::ExpandCollapse;
use crate::component::filesystem::FsLayer;
use crate::state::action::ActionMap;

mod quit;
pub mod file;
pub mod movement;
pub mod tree;

lazy_static! {
	pub static ref ACTION_MAP: ActionMap<FsLayer> = create_action_map();
}

fn create_action_map() -> ActionMap<FsLayer> {
	let mut me = ActionMap::new();
	me.add_char_mapping(' ', ExpandCollapse);
	me.add_char_mapping('J', MoveToNextSibling);
	me.add_char_mapping('K', MoveToPreviousSibling);
	me.add_char_mapping('d', DeleteSelected);
	me.add_char_mapping('h', MoveOrTraverseUpParent);
	me.add_char_mapping('j', MoveDown);
	me.add_char_mapping('k', MoveUp);
	me.add_char_mapping('q', Quit);
	me.add_key_mapping(KeyCode::Down, KeyModifiers::ALT, MoveToNextSibling);
	me.add_key_mapping(KeyCode::Down, KeyModifiers::NONE, MoveDown);
	me.add_key_mapping(KeyCode::Left, KeyModifiers::NONE, MoveOrTraverseUpParent);
	me.add_key_mapping(KeyCode::Up, KeyModifiers::ALT, MoveToPreviousSibling);
	me.add_key_mapping(KeyCode::Up, KeyModifiers::NONE, MoveUp);
	me
}
