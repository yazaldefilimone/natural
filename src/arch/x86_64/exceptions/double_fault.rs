use super::report;
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
  panic!("\n{}", report::abort("double fault", &stack_frame));
}
