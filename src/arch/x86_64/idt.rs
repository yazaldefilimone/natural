use crate::arch::x86_64::{exceptions, interrupts};
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
  static ref IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    exceptions::register(&mut idt);
    interrupts::register(&mut idt);

    idt
  };
}

pub fn init() {
  IDT.load();
}
