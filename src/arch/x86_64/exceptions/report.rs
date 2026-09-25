use core::fmt;
use x86_64::VirtAddr;
use x86_64::structures::idt::InterruptStackFrame;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
  Trap,
  Fault,
  Abort,
}

impl fmt::Display for Kind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Kind::Trap => write!(f, "trap"),
      Kind::Fault => write!(f, "fault"),
      Kind::Abort => write!(f, "abort"),
    }
  }
}

pub struct Report<'a> {
  pub kind: Kind,
  pub name: &'static str,
  pub stack_frame: &'a InterruptStackFrame,
  pub addr: Option<VirtAddr>,
}

impl<'a> Report<'a> {
  pub fn with_addr(mut self, addr: Option<VirtAddr>) -> Self {
    self.addr = addr;
    self
  }
}

pub fn trap<'a>(name: &'static str, stack_frame: &'a InterruptStackFrame) -> Report<'a> {
  Report { kind: Kind::Trap, name, stack_frame, addr: None }
}

pub fn fault<'a>(name: &'static str, stack_frame: &'a InterruptStackFrame) -> Report<'a> {
  Report { kind: Kind::Fault, name, stack_frame, addr: None }
}

pub fn abort<'a>(name: &'static str, stack_frame: &'a InterruptStackFrame) -> Report<'a> {
  Report { kind: Kind::Abort, name, stack_frame, addr: None }
}

impl<'a> fmt::Display for Report<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "{}: {}", self.kind, self.name)?;

    let ip = self.stack_frame.instruction_pointer.as_u64();
    let sp = self.stack_frame.stack_pointer.as_u64();

    if let Some(addr) = self.addr {
      writeln!(f, "  addr: {:#018x}", addr.as_u64())?;
      writeln!(f, "  rip : {:#018x} [cs: {:#06x}]", ip, self.stack_frame.code_segment.0)?;
      write!(f, "  rsp : {:#018x} [rflags: {:#06x}]", sp, self.stack_frame.cpu_flags)?;
    } else {
      writeln!(f)?;
      writeln!(f, "  rip : {:#018x}", ip)?;
      write!(f, "  rsp : {:#018x}", sp)?;
    }
    Ok(())
  }
}
