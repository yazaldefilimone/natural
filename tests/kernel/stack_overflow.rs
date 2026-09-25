#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use natural::arch::x86_64::gdt;
use natural::serial_println;
use natural::testkit::qemu::{self, ExitCode};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
  natural::serial_print!("kernel::stack_overflow...\t");

  gdt::init();
  init_test_idt();

  // dispara um stack overflow
  stack_overflow();

  panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
  stack_overflow(); // para cada recursão, o endereço de retorno é empurrado
  volatile::Volatile::new(0).read(); // previne otimizações de tail recursion
}

lazy_static! {
  static ref TEST_IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    unsafe {
      idt
        .double_fault
        .set_handler_fn(test_double_fault_handler)
        .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }

    idt
  };
}

pub fn init_test_idt() {
  TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
  _stack_frame: InterruptStackFrame,
  _error_code: u64,
) -> ! {
  serial_println!("[ok]");
  qemu::exit(ExitCode::Success);
  loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  natural::testkit::panic(info)
}
