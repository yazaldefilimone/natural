use natural::drivers::video::vga::{BUFFER_HEIGHT, BUFFER_WIDTH, WRITER};

#[test_case]
fn wraps_line() {
  let mut writer = WRITER.lock();
  writer.clear();
  for _ in 0..BUFFER_WIDTH + 5 {
    writer.write_byte(b'a');
  }
  assert_eq!(writer.column(), 5);
}

#[test_case]
fn scrolls_screen() {
  let mut writer = WRITER.lock();
  writer.clear();
  writer.write_str("line 1\nline 2");
  writer.scroll();
  let cell = writer.cell(BUFFER_HEIGHT - 3, 0);
  assert_eq!(cell.ascii, b'l');
}
