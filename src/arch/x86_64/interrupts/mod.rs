pub mod keyboard;
pub mod pic;
pub mod timer;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
  Timer = pic::PRIMARY_OFFSET,
  Keyboard,
}

impl InterruptIndex {
  pub const fn as_u8(self) -> u8 {
    self as u8
  }

  pub const fn as_usize(self) -> usize {
    self as usize
  }
}
