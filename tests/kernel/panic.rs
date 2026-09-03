#![no_std]
#![no_main]

use core::panic::PanicInfo;
use natural::serial_println;
use natural::testkit::qemu::{self, ExitCode};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
  should_fail();
  serial_println!("[test did not panic]");
  qemu::exit(ExitCode::Failed);
  loop {}
}

fn should_fail() {
  natural::serial_print!("kernel::panic::should_fail...\t");
  assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  serial_println!("[ok]");
  qemu::exit(ExitCode::Success);
  loop {}
}
