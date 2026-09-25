pub mod com;
pub mod ports;
pub use com::COM1;

#[doc(hidden)]
pub fn print(args: core::fmt::Arguments) {
  use core::fmt::Write;
  COM1
    .lock()
    .write_fmt(args)
    .expect("printing to serial failed");
}
