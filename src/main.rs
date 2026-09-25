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

  // fn stack_overflow() {
  //   stack_overflow(); // para cada recursão, o endereço de retorno é empurrado
  // }

  // // dispara um stack overflow
  // stack_overflow();

  #[cfg(test)]
  test_main();

  println!("It's don't crash!");
  loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  natural::testkit::panic(info)
}
