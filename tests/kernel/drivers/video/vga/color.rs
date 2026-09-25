use natural::drivers::video::vga::WRITER;
use natural::drivers::video::vga::color::{Color, ColorCode};
use x86_64::instructions::interrupts;

#[test_case]
fn keeps_color() {
  interrupts::without_interrupts(|| {
    let writer = WRITER.lock();
    assert_eq!(writer.color(), ColorCode::new(Color::Yellow, Color::Black));
  });
}

#[test_case]
fn changes_color() {
  interrupts::without_interrupts(|| {
    let mut writer = WRITER.lock();
    let new_color = ColorCode::new(Color::LightGreen, Color::Black);
    writer.set_color(new_color);
    assert_eq!(writer.color(), new_color);
    // Restore default color
    writer.set_color(ColorCode::new(Color::Yellow, Color::Black));
  });
}
