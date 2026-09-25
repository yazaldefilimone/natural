pub mod keyboard;
pub mod pic;
pub mod timer;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
  Timer = pic::PRIMARY_OFFSET,
  Keyboard,
}
type Handler = extern "x86-interrupt" fn(InterruptStackFrame);

impl InterruptIndex {
  #[inline]
  pub const fn as_u8(self) -> u8 {
    self as u8
  }

  #[inline]
  pub const fn as_usize(self) -> usize {
    self as usize
  }

  #[inline]
  pub fn set_handler(self, idt: &mut InterruptDescriptorTable, handler: Handler) {
    idt[self.as_u8()].set_handler_fn(handler);
  }
}

#[inline]
pub fn register(idt: &mut InterruptDescriptorTable) {
  InterruptIndex::Timer.set_handler(idt, timer::handler);
  InterruptIndex::Keyboard.set_handler(idt, keyboard::handler);
}
