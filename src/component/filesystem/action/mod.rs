use lazy_static::lazy_static;

use crate::component::filesystem::action::file::{CreateDirectory, CreateFile, DeleteSelected};
use crate::component::filesystem::action::movement::{HeightRatio, MoveDown, MoveOrTraverseUpParent, MoveToNextSibling, MoveToPreviousSibling, MoveUp, RepeatMovement};
use crate::component::filesystem::action::quit::Quit;
use crate::component::filesystem::action::tree::{ExpandCollapse, RefreshChildrenOfSelected};
use crate::component::filesystem::FsLayer;
use crate::input::keymap::KeyMap;
use crate::state::action::Action;

mod quit;
pub mod file;
pub mod movement;
pub mod tree;

type ActionKeyMap = KeyMap<Box<dyn Action<FsLayer> + Sync>>;

lazy_static! {
	pub static ref ACTION_MAP: ActionKeyMap = create_action_map();
}

fn create_action_map() -> ActionKeyMap {
	let mut me = ActionKeyMap::new();
	
	map(&mut me, " ", ExpandCollapse);
	map(&mut me, "d", DeleteSelected);
	map(&mut me, "h", MoveOrTraverseUpParent);
	map(&mut me, "j", MoveDown);
	map(&mut me, "J", MoveToNextSibling);
	map(&mut me, "k", MoveUp);
	map(&mut me, "K", MoveToPreviousSibling);
	map(&mut me, "nf", CreateFile);
	map(&mut me, "nd", CreateDirectory);
	map(&mut me, "q", Quit);
	map(&mut me, "r", RefreshChildrenOfSelected);
	
	map(&mut me, "<C-b>", RepeatMovement::new(MoveUp, HeightRatio(1)));
	map(&mut me, "<C-c>", Quit);
	map(&mut me, "<C-d>", RepeatMovement::new(MoveDown, HeightRatio(2)));
	map(&mut me, "<C-f>", RepeatMovement::new(MoveDown, HeightRatio(1)));
	map(&mut me, "<C-u>", RepeatMovement::new(MoveUp, HeightRatio(2)));
	
	map(&mut me, "<Down>", MoveDown);
	map(&mut me, "<A-Down>", MoveToNextSibling);
	map(&mut me, "<Left>", MoveOrTraverseUpParent);
	map(&mut me, "<Up>", MoveUp);
	map(&mut me, "<A-Up>", MoveToPreviousSibling);
	
	me
}

fn map(map: &mut ActionKeyMap, key_binding_str: &str, action: impl Action<FsLayer> + Sync + 'static) {
	// Panic on error, since we are hard-coding the key bindings.
	if let Err(err) = map.insert(key_binding_str, Box::new(action)) {
		panic!("Failed to insert key binding: {:?}", err);
	}
}
