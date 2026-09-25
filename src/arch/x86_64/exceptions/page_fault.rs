use super::report;
use crate::println;
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

pub extern "x86-interrupt" fn handler(
  stack_frame: InterruptStackFrame,
  _error_code: PageFaultErrorCode,
) {
  println!("{}", report::fault("page fault", &stack_frame).with_addr(Cr2::read().ok()));
  crate::arch::hlt_loop();
}
