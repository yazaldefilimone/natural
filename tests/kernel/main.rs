#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(natural::testkit::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod arch;
mod boot;
mod drivers;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
  natural::init();
  test_main();
  loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  natural::testkit::panic(info)
}
