use super::report;
use crate::println;
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn handler(stack_frame: InterruptStackFrame) {
  println!("{}", report::trap("breakpoint", &stack_frame));
}
