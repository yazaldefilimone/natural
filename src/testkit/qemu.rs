const EXIT_PORT: u16 = 0xf4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ExitCode {
  Success = 0x10,
  Failed = 0x11,
}

pub fn exit(code: ExitCode) {
  use x86_64::instructions::port::Port;
  unsafe {
    let mut port = Port::new(EXIT_PORT);
    port.write(code as u32);
  }
}
