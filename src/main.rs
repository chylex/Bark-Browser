#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

#![warn(clippy::arithmetic_side_effects)]
#![warn(clippy::as_underscore)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::default_numeric_fallback)]
#![warn(clippy::empty_structs_with_brackets)]
#![warn(clippy::expect_used)]
#![warn(clippy::float_cmp_const)]
#![warn(clippy::fn_to_numeric_cast_any)]
#![warn(clippy::format_push_string)]
#![warn(clippy::get_unwrap)]
#![warn(clippy::if_then_some_else_none)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::lossy_float_literal)]
#![warn(clippy::mem_forget)]
#![warn(clippy::missing_assert_message)]
#![warn(clippy::mixed_read_write_in_expression)]
#![warn(clippy::panic_in_result_fn)]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]
#![warn(clippy::rc_buffer)]
#![warn(clippy::rc_mutex)]
#![warn(clippy::ref_patterns)]
#![warn(clippy::rest_pat_in_fully_bound_structs)]
#![warn(clippy::self_named_module_files)]
#![warn(clippy::str_to_string)]
#![warn(clippy::string_add)]
#![warn(clippy::string_slice)]
#![warn(clippy::string_to_string)]
#![warn(clippy::todo)]
#![warn(clippy::try_err)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(clippy::unseparated_literal_suffix)]
#![warn(clippy::unwrap_in_result)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::use_debug)]
#![warn(clippy::verbose_file_reads)]
#![warn(clippy::wildcard_enum_match_arm)]

#![allow(clippy::bool_to_int_with_if)]
#![allow(clippy::map_unwrap_or)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::option_if_let_else)]
#![allow(clippy::redundant_else)]

use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::ExitCode;

mod app;
mod component;
mod file;
mod input;
mod state;
mod util;

#[allow(clippy::print_stdout)]
fn main() -> Result<ExitCode, Box<dyn Error>> {
	let args = env::args_os().skip(1).collect::<Vec<_>>();
	if args.len() > 1 {
		println!("Too many arguments!");
		return Ok(ExitCode::SUCCESS);
	}
	
	return if let Some(path) = args.get(0).map(PathBuf::from).or_else(|| env::current_dir().ok()) {
		app::run(&path)?;
		Ok(ExitCode::SUCCESS)
	} else {
		println!("Invalid path!");
		Ok(ExitCode::FAILURE)
	}
}
