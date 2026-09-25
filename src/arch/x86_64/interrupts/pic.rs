use pic8259::ChainedPics;
use spin::Mutex;
pub const PRIMARY_OFFSET: u8 = 32;
pub const SECONDARY_OFFSET: u8 = PRIMARY_OFFSET + 8;

pub static CONTROLLER: Mutex<ChainedPics> =
  Mutex::new(unsafe { ChainedPics::new(PRIMARY_OFFSET, SECONDARY_OFFSET) });

pub fn init() {
  unsafe {
    CONTROLLER.lock().initialize();
  }
}

pub fn send_eoi(interrupt_id: u8) {
  unsafe {
    CONTROLLER.lock().notify_end_of_interrupt(interrupt_id);
  }
}
