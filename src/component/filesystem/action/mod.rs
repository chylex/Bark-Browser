use lazy_static::lazy_static;

use crate::component::filesystem::action::application::{Quit, RedrawScreen};
use crate::component::filesystem::action::count::PushCountDigit;
use crate::component::filesystem::action::file::{CreateDirectoryInParentOfSelectedEntry, CreateDirectoryInSelectedDirectory, CreateFileInParentOfSelectedEntry, CreateFileInSelectedDirectory, DeleteSelectedEntry, EditSelectedEntry, RenameSelectedEntry};
use crate::component::filesystem::action::movement::{CollapseSelectedOr, ExpandSelectedOr, MoveBetweenFirstAndLastSibling, MoveDown, MovementWithCountFactory, MovementWithFallbackFactory, MoveOrTraverseUpParent, MoveToFirst, MoveToLast, MoveToLineOr, MoveToNextSibling, MoveToParent, MoveToPreviousSibling, MoveUp, ScreenHeightRatio};
use crate::component::filesystem::action::tree::{ExpandCollapse, RefreshChildrenOfSelected};
use crate::component::filesystem::FsLayer;
use crate::input::keymap::KeyMap;
use crate::state::action::Action;

mod application;
mod count;
mod file;
mod movement;
mod tree;

type ActionKeyMap = KeyMap<Box<dyn Action<FsLayer> + Sync>>;

lazy_static! {
	pub static ref ACTION_MAP: ActionKeyMap = create_action_map();
}

fn create_action_map() -> ActionKeyMap {
	let mut me = ActionKeyMap::new();
	
	map(&mut me, "0", PushCountDigit(0));
	map(&mut me, "1", PushCountDigit(1));
	map(&mut me, "2", PushCountDigit(2));
	map(&mut me, "3", PushCountDigit(3));
	map(&mut me, "4", PushCountDigit(4));
	map(&mut me, "5", PushCountDigit(5));
	map(&mut me, "6", PushCountDigit(6));
	map(&mut me, "7", PushCountDigit(7));
	map(&mut me, "8", PushCountDigit(8));
	map(&mut me, "9", PushCountDigit(9));
	
	map(&mut me, "af", CreateFileInSelectedDirectory);
	map(&mut me, "ad", CreateDirectoryInSelectedDirectory);
	map(&mut me, "e", EditSelectedEntry);
	map(&mut me, "d", DeleteSelectedEntry);
	map(&mut me, "gg", MoveToLineOr(MoveToFirst));
	map(&mut me, "G", MoveToLineOr(MoveToLast));
	map(&mut me, "h", CollapseSelectedOr(MoveToParent));
	map(&mut me, "H", MoveOrTraverseUpParent);
	map(&mut me, "if", CreateFileInSelectedDirectory);
	map(&mut me, "id", CreateDirectoryInSelectedDirectory);
	map(&mut me, "j", MoveDown);
	map(&mut me, "J", MoveToNextSibling.with_fallback(MoveDown));
	map(&mut me, "k", MoveUp);
	map(&mut me, "K", MoveToPreviousSibling.with_fallback(MoveUp));
	map(&mut me, "l", ExpandSelectedOr(MoveDown));
	map(&mut me, "of", CreateFileInParentOfSelectedEntry);
	map(&mut me, "od", CreateDirectoryInParentOfSelectedEntry);
	map(&mut me, "q", Quit);
	map(&mut me, "r", RenameSelectedEntry { prefill: true });
	map(&mut me, "R", RenameSelectedEntry { prefill: false });
	
	map(&mut me, "%", MoveBetweenFirstAndLastSibling);
	
	map(&mut me, "<Ctrl-B>", MoveUp.with_custom_count(ScreenHeightRatio(1)));
	map(&mut me, "<Ctrl-C>", Quit);
	map(&mut me, "<Ctrl-D>", MoveDown.with_default_count(ScreenHeightRatio(2)));
	map(&mut me, "<Ctrl-F>", MoveDown.with_custom_count(ScreenHeightRatio(1)));
	map(&mut me, "<Ctrl-L>", RedrawScreen);
	map(&mut me, "<Ctrl-N>", MoveDown);
	map(&mut me, "<Ctrl-P>", MoveUp);
	map(&mut me, "<Ctrl-U>", MoveUp.with_default_count(ScreenHeightRatio(2)));
	
	map(&mut me, "<Space>", ExpandCollapse { default_depth: 1 });
	map(&mut me, "<Ctrl-Space>", ExpandCollapse { default_depth: 1000 });
	
	map(&mut me, "<Down>", MoveDown);
	map(&mut me, "<Shift-Down>", MoveDown.with_custom_count(ScreenHeightRatio(1)));
	map(&mut me, "<Alt-Down>", MoveToNextSibling.with_fallback(MoveDown));
	
	map(&mut me, "<Up>", MoveUp);
	map(&mut me, "<Shift-Up>", MoveUp.with_custom_count(ScreenHeightRatio(1)));
	map(&mut me, "<Alt-Up>", MoveToPreviousSibling.with_fallback(MoveUp));
	
	map(&mut me, "<Left>", CollapseSelectedOr(MoveToParent));
	map(&mut me, "<Alt-Left>", MoveOrTraverseUpParent);
	
	map(&mut me, "<Right>", ExpandSelectedOr(MoveDown));
	
	map(&mut me, "<Del>", DeleteSelectedEntry);
	
	map(&mut me, "<PageDown>", MoveDown.with_custom_count(ScreenHeightRatio(1)));
	map(&mut me, "<PageUp>", MoveUp.with_custom_count(ScreenHeightRatio(1)));
	
	map(&mut me, "<F2>", RenameSelectedEntry { prefill: true });
	map(&mut me, "<Shift-F2>", RenameSelectedEntry { prefill: false });
	
	map(&mut me, "<F5>", RefreshChildrenOfSelected);
	
	me
}

fn map(map: &mut ActionKeyMap, key_binding_str: &str, action: impl Action<FsLayer> + Sync + 'static) {
	// Panic on error, since we are hard-coding the key bindings.
	if let Err(err) = map.insert(key_binding_str, Box::new(action)) {
		panic!("Failed to insert key binding '{}'. {}", err.sequence(), err.error());
	}
}
