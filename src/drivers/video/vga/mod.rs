pub mod buffer;
pub mod color;
mod macros;
mod writer;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;
pub const VGA_TEXT_BUFFER_ADDRESS: usize = 0xb8000;

pub use macros::*;
pub use writer::*;
