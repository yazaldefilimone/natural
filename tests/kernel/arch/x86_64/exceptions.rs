#[test_case]
fn breakpoint() {
  x86_64::instructions::interrupts::int3();
}
