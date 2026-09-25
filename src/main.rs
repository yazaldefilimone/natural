#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(natural::testkit::runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use natural::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
  println!("Hello Natural!");

  natural::init();

  #[cfg(test)]
  test_main();

  println!("It's don't crash!");

  natural::arch::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  natural::arch::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  natural::testkit::panic(info)
}
