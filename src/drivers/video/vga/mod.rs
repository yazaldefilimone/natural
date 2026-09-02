pub mod buffer;
pub mod colors;
mod macros;
mod writer;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

pub use macros::*;
pub use writer::*;
