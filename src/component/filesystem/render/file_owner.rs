use ratatui::buffer::Buffer;
use ratatui::style::{Color, Style};
use ratatui::text::Span;

use crate::component::filesystem::ColumnWidths;
use crate::component::filesystem::render::column;
use crate::file::{FileOwnerName, FileOwnerNameCache};

#[cfg(unix)]
pub const fn visible() -> bool { true }

#[cfg(not(unix))]
pub const fn visible() -> bool { false }

#[allow(clippy::similar_names)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn print_user_group(buf: &mut Buffer, x: u16, y: u16, uid: Option<u32>, gid: Option<u32>, name_cache: &mut FileOwnerNameCache, column_widths: &ColumnWidths) {
	print_name(buf, x, y, name_cache.get_user(uid), column_widths.user);
	let x = x.saturating_add(column_widths.user).saturating_add(1);
	print_name(buf, x, y, name_cache.get_group(gid), column_widths.group);
}

fn print_name(buf: &mut Buffer, x: u16, y: u16, name: &FileOwnerName, column_width: u16) {
	let style = match name {
		FileOwnerName::Named(_)   => Style::default(),
		FileOwnerName::Numeric(_) => Style::default().fg(Color::Indexed(248 /* Grey66 */)),
		FileOwnerName::Unknown    => Style::default().fg(Color::DarkGray),
	};
	
	column::print_fixed_width_cell(buf, x, y, column_width, vec![
		Span::styled(name, style),
	]);
}
