use std::path::Path;

use crate::component::filesystem::ActionKeyMap;

pub struct StateInitializer<'a> {
	pub filesystem_start_path: &'a Path,
	pub filesystem_action_map: &'static ActionKeyMap,
}
