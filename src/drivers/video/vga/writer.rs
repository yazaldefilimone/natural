use crate::{
  console::Console,
  drivers::video::vga::{
    BUFFER_HEIGHT, BUFFER_WIDTH,
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
  fn new_line(&mut self) {
    for row in 1..BUFFER_HEIGHT {
      for col in 0..BUFFER_WIDTH {
        let character = self.buffer.cells[row][col].read();
        self.buffer.cells[row - 1][col].write(character);
      }
    }
    self.clear_row(BUFFER_HEIGHT - 1);
    self.column = 0;
  }

  fn clear_row(&mut self, row: usize) {
    let blank = Cell {
      ascii: b' ',
      color: self.color,
    };
    for col in 0..BUFFER_WIDTH {
      self.buffer.cells[row][col].write(blank);
    }
  }
}

impl Console for Writer {
  fn write_byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.new_line(),
      byte => {
        if self.column >= BUFFER_WIDTH {
          self.new_line();
        }

        let row = BUFFER_HEIGHT - 1;
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
}

impl fmt::Write for Writer {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.write(s);
    Ok(())
  }
}

lazy_static! {
  pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    column: 0,
    color: ColorCode::new(Color::Yellow, Color::Black),
    buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
  });
}
