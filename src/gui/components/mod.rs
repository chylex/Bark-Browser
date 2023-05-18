//! Components should provide a function for either rendering or printing themselves.
//!
//! Both must set their desired foreground and background colors at the beginning, and should not restore them to the original values.
//! 
//! Rendering functions may affect any part of the terminal, and may leave the cursor in any position.
//! 
//! Printing functions must only affect the rest of the current line or lines below the cursor's initial position, and must leave the cursor
//! after the end of printed content that is intended to remain. Anything past the final cursor position will be overwritten.

mod column;
pub mod date_time;
pub mod file_name;
pub mod file_owner;
pub mod file_permissions;
pub mod file_size;
