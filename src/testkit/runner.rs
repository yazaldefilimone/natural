pub trait Testable {
  fn run(&self);
}

impl<T> Testable for T
where
  T: Fn(),
{
  fn run(&self) {
    crate::serial_print!("{}...\t", core::any::type_name::<T>());
    self();
    crate::serial_println!("[ok]");
  }
}

pub fn runner(tests: &[&dyn Testable]) {
  crate::serial_println!("Running {} tests", tests.len());
  for test in tests {
    test.run();
  }
  super::qemu::exit(super::qemu::ExitCode::Success);
}
