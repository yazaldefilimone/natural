pub mod x86_64;

pub fn hlt_loop() -> ! {
  #[cfg(target_arch = "x86_64")]
  x86_64::hlt_loop();
  #[cfg(target_arch = "x86")]
  x86::hlt_loop();
  #[cfg(target_arch = "arm")]
  arm::hlt_loop();
}
