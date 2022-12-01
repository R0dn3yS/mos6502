bitflags! {
  pub struct CpuFlags: u8 {
    const CARRY = 0b0000_0001;
    const ZERO = 0b0000_0010;
		const INTERRUPT_DISABLE	= 0b0000_0100;
		const DECIMAL_MODE = 0b0000_1000;
		const BREAK = 0b0001_0000;
		const BREAK2 = 0b0010_0000;
		const OVERFLOW = 0b0100_0000;
		const NEGATIV = 0b1000_0000;
  }
}

pub struct CPU {
  pc: u16, // Program Counter
  ac: i8, // Accumulator
  x: u8, // X Register
  y: u8, // Y Register
  sr: u8, // Status Register
  sp: u8, // Stack Pointer
}

impl CPU {
  pub fn new() -> CPU {
    CPU {
      pc: 0,
      ac: 0,
      x: 0,
      y: 0,
      sr: CpuFlags::from_bits_truncate(0b100100),
      sp: 0,
    }
  }
}