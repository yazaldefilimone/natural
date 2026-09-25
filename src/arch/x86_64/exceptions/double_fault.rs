use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
  panic!("exception: double fault\n{:#?}", stack_frame);
}
