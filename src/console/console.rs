pub trait Console {
  fn write_byte(&mut self, byte: u8);

  fn write(&mut self, text: &str) {
    for byte in text.bytes() {
      self.write_byte(byte);
    }
  }
}
