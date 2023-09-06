use lazy_static::lazy_static;

use crate::component::filesystem::{ActionKeyMap, FsLayer};
use crate::component::filesystem::action::application::{EnterCommandMode, Quit, RedrawScreen};
use crate::component::filesystem::action::count::PushCountDigit;
use crate::component::filesystem::action::file::{CreateDirectoryInParentOfSelectedEntry, CreateDirectoryInSelectedDirectory, CreateFileInParentOfSelectedEntry, CreateFileInSelectedDirectory, DeleteSelectedEntry, EditSelectedEntry, RenameSelectedEntry};
use crate::component::filesystem::action::movement::{CollapseSelectedOr, ExpandSelectedOr, MoveBetweenFirstAndLastSibling, MoveDown, MovementWithCountFactory, MovementWithFallbackFactory, MoveOrTraverseUpParent, MoveToFirst, MoveToLast, MoveToLineOr, MoveToNextSibling, MoveToParent, MoveToPreviousSibling, MoveUp, ScreenHeightRatio};
use crate::component::filesystem::action::tree::{ExpandCollapse, RefreshChildrenOfSelected};
use crate::input::keymap::KeyMapInsertError;
use crate::state::action::Action;

lazy_static! {
	pub static ref ACTION_MAP: Result<ActionKeyMap, KeyMapInsertError> = create_action_map();
}

pub fn get_action_map() -> Result<&'static ActionKeyMap, &'static KeyMapInsertError> {
	return ACTION_MAP.as_ref();
}

fn create_action_map() -> Result<ActionKeyMap, KeyMapInsertError> {
	let mut me = ActionKeyMap::new();
	
	map(&mut me, "0", PushCountDigit(0))?;
	map(&mut me, "1", PushCountDigit(1))?;
	map(&mut me, "2", PushCountDigit(2))?;
	map(&mut me, "3", PushCountDigit(3))?;
	map(&mut me, "4", PushCountDigit(4))?;
	map(&mut me, "5", PushCountDigit(5))?;
	map(&mut me, "6", PushCountDigit(6))?;
	map(&mut me, "7", PushCountDigit(7))?;
	map(&mut me, "8", PushCountDigit(8))?;
	map(&mut me, "9", PushCountDigit(9))?;
	
	map(&mut me, "af", CreateFileInSelectedDirectory)?;
	map(&mut me, "ad", CreateDirectoryInSelectedDirectory)?;
	map(&mut me, "e", EditSelectedEntry)?;
	map(&mut me, "d", DeleteSelectedEntry)?;
	map(&mut me, "gg", MoveToLineOr(MoveToFirst))?;
	map(&mut me, "G", MoveToLineOr(MoveToLast))?;
	map(&mut me, "h", CollapseSelectedOr(MoveToParent))?;
	map(&mut me, "H", MoveOrTraverseUpParent)?;
	map(&mut me, "if", CreateFileInSelectedDirectory)?;
	map(&mut me, "id", CreateDirectoryInSelectedDirectory)?;
	map(&mut me, "j", MoveDown)?;
	map(&mut me, "J", MoveToNextSibling.with_fallback(MoveDown))?;
	map(&mut me, "k", MoveUp)?;
	map(&mut me, "K", MoveToPreviousSibling.with_fallback(MoveUp))?;
	map(&mut me, "l", ExpandSelectedOr(MoveDown))?;
	map(&mut me, "of", CreateFileInParentOfSelectedEntry)?;
	map(&mut me, "od", CreateDirectoryInParentOfSelectedEntry)?;
	map(&mut me, "q", Quit)?;
	map(&mut me, "r", RenameSelectedEntry { prefill: true })?;
	map(&mut me, "R", RenameSelectedEntry { prefill: false })?;
	
	map(&mut me, "%", MoveBetweenFirstAndLastSibling)?;
	map(&mut me, ":", EnterCommandMode)?;
	
	map(&mut me, "<Ctrl-B>", MoveUp.with_custom_count(ScreenHeightRatio(1)))?;
	map(&mut me, "<Ctrl-C>", Quit)?;
	map(&mut me, "<Ctrl-D>", MoveDown.with_default_count(ScreenHeightRatio(2)))?;
	map(&mut me, "<Ctrl-F>", MoveDown.with_custom_count(ScreenHeightRatio(1)))?;
	map(&mut me, "<Ctrl-L>", RedrawScreen)?;
	map(&mut me, "<Ctrl-N>", MoveDown)?;
	map(&mut me, "<Ctrl-P>", MoveUp)?;
	map(&mut me, "<Ctrl-U>", MoveUp.with_default_count(ScreenHeightRatio(2)))?;
	
	map(&mut me, "<Space>", ExpandCollapse { default_depth: 1 })?;
	map(&mut me, "<Ctrl-Space>", ExpandCollapse { default_depth: 1000 })?;
	
	map(&mut me, "<Down>", MoveDown)?;
	map(&mut me, "<Shift-Down>", MoveDown.with_custom_count(ScreenHeightRatio(1)))?;
	map(&mut me, "<Alt-Down>", MoveToNextSibling.with_fallback(MoveDown))?;
	
	map(&mut me, "<Up>", MoveUp)?;
	map(&mut me, "<Shift-Up>", MoveUp.with_custom_count(ScreenHeightRatio(1)))?;
	map(&mut me, "<Alt-Up>", MoveToPreviousSibling.with_fallback(MoveUp))?;
	
	map(&mut me, "<Left>", CollapseSelectedOr(MoveToParent))?;
	map(&mut me, "<Alt-Left>", MoveOrTraverseUpParent)?;
	
	map(&mut me, "<Right>", ExpandSelectedOr(MoveDown))?;
	
	map(&mut me, "<Del>", DeleteSelectedEntry)?;
	
	map(&mut me, "<PageDown>", MoveDown.with_custom_count(ScreenHeightRatio(1)))?;
	map(&mut me, "<PageUp>", MoveUp.with_custom_count(ScreenHeightRatio(1)))?;
	
	map(&mut me, "<F2>", RenameSelectedEntry { prefill: true })?;
	map(&mut me, "<Shift-F2>", RenameSelectedEntry { prefill: false })?;
	
	map(&mut me, "<F5>", RefreshChildrenOfSelected)?;
	
	Ok(me)
}

fn map(map: &mut ActionKeyMap, key_binding_str: &str, action: impl Action<FsLayer> + Sync + 'static) -> Result<(), KeyMapInsertError> {
	map.insert(key_binding_str, Box::new(action))
}
