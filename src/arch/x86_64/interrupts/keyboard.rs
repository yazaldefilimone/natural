use crate::arch::x86_64::interrupts::{self, InterruptIndex};
use crate::print;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
use spin::Mutex;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;

pub const DATA_PORT: u16 = 0x60;

static KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
  Mutex::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore));

pub extern "x86-interrupt" fn handler(_stack_frame: InterruptStackFrame) {
  let mut keyboard = KEYBOARD.lock();
  let mut port = Port::new(DATA_PORT);

  let scancode: u8 = unsafe { port.read() };
  if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
    if let Some(key) = keyboard.process_keyevent(key_event) {
      match key {
        DecodedKey::Unicode(character) => print!("{}", character),
        DecodedKey::RawKey(key) => print!("{:?}", key),
      }
    }
  }

  interrupts::pic::send_eoi(InterruptIndex::Keyboard.as_u8());
}
