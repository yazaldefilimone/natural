use core::fmt::Write;
use natural::drivers::video::vga::color::{Color, ColorCode};
use natural::drivers::video::vga::{BUFFER_HEIGHT, WRITER};
use x86_64::instructions::interrupts;

#[test_case]
fn test_println_output() {
  let s = "A single line test string that fits on one line";
  interrupts::without_interrupts(|| {
    let mut writer = WRITER.lock();
    writeln!(writer, "\n{}", s).expect("writeln failed");
    for (i, c) in s.chars().enumerate() {
      let cell = writer.cell(BUFFER_HEIGHT - 2, i);
      assert_eq!(char::from(cell.ascii), c);
    }
  });
}

#[test_case]
fn test_println_many() {
  for _ in 0..200 {
    natural::println!("test_println_many output");
  }
}

#[test_case]
fn writes_char() {
  interrupts::without_interrupts(|| {
    let mut writer = WRITER.lock();
    writer.newline();
    writer.write_byte(b'X');
    let cell = writer.cell(BUFFER_HEIGHT - 1, 0);
    assert_eq!(cell.ascii, b'X');
    writer.newline();
  });
}

#[test_case]
fn writes_with_custom_color() {
  interrupts::without_interrupts(|| {
    let mut writer = WRITER.lock();
    let original_color = writer.color();
    let test_color = ColorCode::new(Color::Cyan, Color::DarkGray);

    writer.set_color(test_color);
    writer.newline();
    writer.write_byte(b'Z');

    let cell = writer.cell(BUFFER_HEIGHT - 1, 0);
    assert_eq!(cell.ascii, b'Z');
    assert_eq!(cell.color, test_color);

    writer.set_color(original_color);
    writer.newline();
  });
}
