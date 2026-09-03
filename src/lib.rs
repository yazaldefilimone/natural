#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testkit::runner)]
#![reexport_test_harness_main = "test_main"]

pub mod console;
pub mod drivers;
pub mod testkit;

#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
  test_main();
  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  testkit::panic(info)
}
