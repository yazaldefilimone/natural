use crate::println;
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
  println!("exception: breakpoint\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(
  stack_frame: InterruptStackFrame,
  _error_code: u64,
) -> ! {
  panic!("exception: double fault\n{:#?}", stack_frame);
}
