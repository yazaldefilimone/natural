use core::panic::PanicInfo;

pub fn panic(info: &PanicInfo) -> ! {
  crate::serial_println!("[failed]");
  crate::serial_println!("Error: {}", info);

  super::qemu::exit(super::qemu::ExitCode::Failed);

  loop {}
}
