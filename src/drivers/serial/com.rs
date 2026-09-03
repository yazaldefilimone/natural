use super::ports::COM1_ADDRESS;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::{Config, Uart16550Tty, backend::PioBackend};

lazy_static! {
  pub static ref COM1: Mutex<Uart16550Tty<PioBackend>> = Mutex::new(unsafe {
    let config = Config::default();
    Uart16550Tty::new_port(COM1_ADDRESS, config).expect("failed to initialize COM1")
  });
}
