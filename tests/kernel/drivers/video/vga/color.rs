use natural::drivers::video::vga::WRITER;
use natural::drivers::video::vga::colors::{Color, ColorCode};

#[test_case]
fn keeps_color() {
  let writer = WRITER.lock();
  assert_eq!(writer.color(), ColorCode::new(Color::Yellow, Color::Black));
}

#[test_case]
fn changes_color() {
  let mut writer = WRITER.lock();
  let new_color = ColorCode::new(Color::LightGreen, Color::Black);
  writer.set_color(new_color);
  assert_eq!(writer.color(), new_color);
  // Restore default color
  writer.set_color(ColorCode::new(Color::Yellow, Color::Black));
}
