pub mod breakpoint;
pub mod double_fault;
pub mod page_fault;

use crate::arch::x86_64::gdt;
use x86_64::structures::idt::InterruptDescriptorTable;

pub fn register(idt: &mut InterruptDescriptorTable) {
  idt.breakpoint.set_handler_fn(breakpoint::handler);
  idt.page_fault.set_handler_fn(page_fault::handler);

  unsafe {
    let options = idt.double_fault.set_handler_fn(double_fault::handler);
    options.set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
  }
}
