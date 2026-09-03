use crate::{
  console::Console,
  drivers::video::vga::{
    self,
    buffer::{Buffer, Cell},
    colors::{Color, ColorCode},
  },
};

use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

pub struct Writer {
  column: usize,
  color: ColorCode,
  buffer: &'static mut Buffer,
}

impl Writer {
  pub fn new(color: ColorCode, buffer: &'static mut Buffer) -> Self {
    Writer {
      column: 0,
      color,
      buffer,
    }
  }

  pub fn write_byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.newline(),
      byte => {
        if self.column >= vga::BUFFER_WIDTH {
          self.newline();
        }

        let row = vga::BUFFER_HEIGHT - 1;
        let column = self.column;

        let cell = Cell {
          ascii: byte,
          color: self.color,
        };
        self.buffer.cells[row][column].write(cell);
        self.column += 1;
      },
    }
  }

  pub fn write_str(&mut self, text: &str) {
    for byte in text.bytes() {
      match byte {
        0x20..=0x7e | b'\n' => self.write_byte(byte),
        _ => self.write_byte(0xfe),
      }
    }
  }

  pub fn newline(&mut self) {
    self.scroll();
    self.column = 0;
  }

  pub fn scroll(&mut self) {
    for row in 1..vga::BUFFER_HEIGHT {
      for col in 0..vga::BUFFER_WIDTH {
        let character = self.buffer.cells[row][col].read();
        self.buffer.cells[row - 1][col].write(character);
      }
    }
    self.clear_row(vga::BUFFER_HEIGHT - 1);
  }

  pub fn clear(&mut self) {
    for row in 0..vga::BUFFER_HEIGHT {
      self.clear_row(row);
    }
    self.column = 0;
  }

  fn clear_row(&mut self, row: usize) {
    let blank = Cell {
      ascii: b' ',
      color: self.color,
    };
    for col in 0..vga::BUFFER_WIDTH {
      self.buffer.cells[row][col].write(blank);
    }
  }

  pub fn color(&self) -> ColorCode {
    self.color
  }

  pub fn set_color(&mut self, color: ColorCode) {
    self.color = color;
  }

  pub fn column(&self) -> usize {
    self.column
  }

  pub fn cell(&self, row: usize, col: usize) -> Cell {
    self.buffer.cells[row][col].read()
  }
}

lazy_static! {
  pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    column: 0,
    color: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(vga::VGA_TEXT_BUFFER_ADDRESS as *mut Buffer) },
  });
}

impl Console for Writer {
  fn write_byte(&mut self, byte: u8) {
    Writer::write_byte(self, byte);
  }

  fn write(&mut self, text: &str) {
    self.write_str(text);
  }
}

impl fmt::Write for Writer {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    Writer::write_str(self, s);
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test_case]
  fn creates_writer() {
    let writer = WRITER.lock();
    assert_eq!(writer.column(), 0);
  }
}
