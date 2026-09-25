use crate::arch::x86_64::exceptions::breakpoint_handler;
use crate::arch::x86_64::exceptions::double_fault_handler;
use crate::arch::x86_64::gdt;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
  static ref IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.double_fault.set_handler_fn(double_fault_handler);
    unsafe {
      idt
        .double_fault
        .set_handler_fn(double_fault_handler)
        .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }
    idt
  };
}

pub fn init() {
  IDT.load();
}
