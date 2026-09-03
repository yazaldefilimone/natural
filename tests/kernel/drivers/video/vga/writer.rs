use natural::drivers::video::vga::{BUFFER_HEIGHT, WRITER};
use natural::println;

#[test_case]
fn writes_char() {
  let mut writer = WRITER.lock();
  writer.newline();
  writer.write_byte(b'X');
  let cell = writer.cell(BUFFER_HEIGHT - 1, 0);
  assert_eq!(cell.ascii, b'X');
  writer.newline();
}

#[test_case]
fn writes_string() {
  let s = "Hello Natural OS";
  println!("{}", s);
  for (i, c) in s.chars().enumerate() {
    let cell = WRITER.lock().cell(BUFFER_HEIGHT - 2, i);
    assert_eq!(char::from(cell.ascii), c);
  }
}

#[test_case]
fn prints_text() {
  println!("prints_text output");
}

#[test_case]
fn prints_format() {
  println!("Formatted: {} + {} = {}", 2, 2, 4);
}

#[test_case]
fn prints_many_lines() {
  for _ in 0..200 {
    println!("prints_many_lines output");
  }
}
