use crate::arch::x86_64::interrupts::{self, InterruptIndex};
use crate::print;
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn handler(_stack_frame: InterruptStackFrame) {
  print!(".");
  interrupts::pic::send_eoi(InterruptIndex::Timer.as_u8());
}
