use crate::drivers::video::vga::color;
use volatile::Volatile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Cell {
  pub ascii: u8,
  pub color: color::ColorCode,
}

#[repr(transparent)]
pub struct Buffer {
  pub cells: [[Volatile<Cell>; super::BUFFER_WIDTH]; super::BUFFER_HEIGHT],
}
