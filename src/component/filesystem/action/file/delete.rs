use std::io;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, Instant};

use ratatui::style::Color;
use ratatui::text::Line;
use slab_tree::NodeId;

use crate::component::dialog::message::MessageDialogLayer;
use crate::component::filesystem::action::file::{FileNode, get_entry_kind_name, get_selected_file};
use crate::component::filesystem::event::FsLayerEvent;
use crate::component::filesystem::FsLayer;
use crate::file::{FileEntry, FileKind};
use crate::state::action::{Action, ActionResult};
use crate::state::Environment;

pub struct DeleteSelectedEntry;

impl Action<FsLayer> for DeleteSelectedEntry {
	fn perform(&self, layer: &mut FsLayer, _environment: &Environment) -> ActionResult {
		if let Some(FileNode { node, entry, path }) = get_selected_file(layer) {
			ActionResult::PushLayer(Box::new(create_delete_confirmation_dialog(layer, node.node_id(), entry, path.to_owned())))
		} else {
			ActionResult::Nothing
		}
	}
}

fn create_delete_confirmation_dialog<'a>(layer: &FsLayer, view_node_id: NodeId, entry: &FileEntry, path: PathBuf) -> MessageDialogLayer<'a> {
	let y = layer.dialog_y();
	let pending_events = Rc::clone(&layer.pending_events);
	
	let total_files = if matches!(entry.kind(), FileKind::Directory) {
		count_files(path.clone())
	} else {
		CountFiles { files: 1, directories: 0 }.into_result(CountFilesResultKind::Success)
	};
	
	MessageDialogLayer::build()
		.y(y)
		.color(Color::LightRed)
		.title(format!("Delete {}", get_entry_kind_name(entry)))
		.message(vec![
			Line::from(format!("Permanently delete {}?", path.to_string_lossy())),
			Line::from(format!("This will affect {}.", total_files.describe())),
		])
		.yes_no(Box::new(move || {
			match delete_path_recursively(&path) {
				Ok(_) => {
					FsLayerEvent::DeleteViewNode(view_node_id).enqueue(&pending_events);
					ActionResult::PopLayer
				}
				Err(e) => {
					ActionResult::ReplaceLayer(Box::new(MessageDialogLayer::generic_error(y.saturating_add(1), e.to_string())))
				}
			}
		}))
}

const MAX_COUNT_TIME: Duration = Duration::from_secs(5);

#[allow(clippy::iter_with_drain, clippy::needless_collect)]
fn count_files(path: PathBuf) -> CountFilesResult {
	let start_time = Instant::now();
	
	let mut remaining_directories = vec![path];
	let mut count = CountFiles { files: 0, directories: 1 };
	let mut errors = 0_usize;
	
	while let Some(path) = remaining_directories.pop() {
		if count.process_directory(&path, &mut remaining_directories).is_err() {
			errors = errors.saturating_add(1);
		}
		
		if start_time.elapsed() >= MAX_COUNT_TIME {
			return count.into_result(CountFilesResultKind::Timeout);
		}
	}
	
	if errors == 0 {
		count.into_result(CountFilesResultKind::Success)
	} else {
		count.into_result(CountFilesResultKind::WithErrors(errors))
	}
}

struct CountFiles {
	files: usize,
	directories: usize,
}

impl CountFiles {
	fn process_directory(&mut self, path: &Path, found_directories: &mut Vec<PathBuf>) -> io::Result<()> {
		for entry in path.read_dir()? {
			let entry = entry?;
			
			if entry.file_type()?.is_dir() {
				self.directories = self.directories.saturating_add(1);
				found_directories.push(entry.path());
			} else {
				self.files = self.files.saturating_add(1);
			}
		}
		
		Ok(())
	}
	
	const fn into_result(self, kind: CountFilesResultKind) -> CountFilesResult {
		CountFilesResult { kind, count: self }
	}
}

struct CountFilesResult {
	kind: CountFilesResultKind,
	count: CountFiles,
}

impl CountFilesResult {
	fn describe(&self) -> String {
		match &self.kind {
			CountFilesResultKind::Success => self.describe_files_and_directories(),
			CountFilesResultKind::Timeout => format!("at least {} (count terminated due to timeout)", self.describe_files_and_directories()),
			CountFilesResultKind::WithErrors(error_count) => {
				let pluralized_errors = if *error_count == 1 { "error" } else { "errors" };
				format!("at least {} (count is incomplete due to {} I/O {})", self.describe_files_and_directories(), error_count, pluralized_errors)
			}
		}
	}
	
	fn describe_files_and_directories(&self) -> String {
		let files = self.count.files;
		let directories = self.count.directories;
		
		let pluralized_files = if files == 1 { "file" } else { "files" };
		let pluralized_directories = if directories == 1 { "directory" } else { "directories" };
		
		if files > 0 && directories > 0 {
			format!("{files} {pluralized_files} and {directories} {pluralized_directories}")
		} else if files > 0 {
			format!("{files} {pluralized_files}")
		} else {
			format!("{directories} {pluralized_directories}")
		}
	}
}

enum CountFilesResultKind {
	Success,
	Timeout,
	WithErrors(usize),
}

fn delete_path_recursively(path: impl AsRef<Path>) -> io::Result<()> {
	if path.as_ref().is_dir() {
		std::fs::remove_dir_all(path)
	} else {
		std::fs::remove_file(path)
	}
}
