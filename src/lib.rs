#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testkit::runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod arch;
pub mod console;
pub mod drivers;
pub mod testkit;

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
  init();

  test_main();
  arch::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  testkit::panic(info)
}

pub fn init() {
  arch::x86_64::gdt::init();
  arch::x86_64::idt::init();
  arch::x86_64::interrupts::pic::init();
  x86_64::instructions::interrupts::enable();
}
