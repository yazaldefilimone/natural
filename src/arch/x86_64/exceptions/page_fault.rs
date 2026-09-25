use crate::println;
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

pub extern "x86-interrupt" fn handler(
  stack_frame: InterruptStackFrame,
  error_code: PageFaultErrorCode,
) {
  println!("exception: page fault");
  println!("accessed address: {:?}", Cr2::read());
  println!("error code: {:?}", error_code);
  println!("{:#?}", stack_frame);
  crate::arch::hlt_loop();
}
