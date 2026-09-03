#[macro_export]
macro_rules! serial_print {
  ($($arg:tt)*) => {
    $crate::drivers::serial::print(format_args!($($arg)*))
  };
}

#[macro_export]
macro_rules! serial_println {
  () => {
    $crate::serial_print!("\n")
  };

  ($fmt:expr) => {
    $crate::serial_print!(concat!($fmt, "\n"))
  };

  ($fmt:expr, $($arg:tt)*) => {
    $crate::serial_print!(concat!($fmt, "\n"), $($arg)*)
  };
}
