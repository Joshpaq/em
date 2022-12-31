use std::fmt;

// PIN MAPPING
// addr pins (out)
const ADDR_00: u64 = 1 << 0;
const ADDR_01: u64 = 1 << 1;
const ADDR_02: u64 = 1 << 2;
const ADDR_03: u64 = 1 << 3;
const ADDR_04: u64 = 1 << 4;
const ADDR_05: u64 = 1 << 5;
const ADDR_06: u64 = 1 << 6;
const ADDR_07: u64 = 1 << 7;
const ADDR_08: u64 = 1 << 8;
const ADDR_09: u64 = 1 << 9;
const ADDR_10: u64 = 1 << 10;
const ADDR_11: u64 = 1 << 11;
const ADDR_12: u64 = 1 << 12;
const ADDR_13: u64 = 1 << 13;
const ADDR_14: u64 = 1 << 14;
const ADDR_15: u64 = 1 << 15;
const ADDR_MASK: u64 = ADDR_00 | ADDR_01 | ADDR_02 | ADDR_03 | ADDR_04 | ADDR_05 | ADDR_06 | ADDR_07 | ADDR_08 | ADDR_09 | ADDR_10 | ADDR_11 | ADDR_12 | ADDR_13 | ADDR_14 | ADDR_15;

// data pins (in/out)
const DATA_0: u64 = 1 << 16;
const DATA_1: u64 = 1 << 17;
const DATA_2: u64 = 1 << 18;
const DATA_3: u64 = 1 << 19;
const DATA_4: u64 = 1 << 20;
const DATA_5: u64 = 1 << 21;
const DATA_6: u64 = 1 << 22;
const DATA_7: u64 = 1 << 23;
const DATA_MASK: u64 = DATA_0 | DATA_1 | DATA_2 | DATA_3 | DATA_4 | DATA_5 | DATA_6 | DATA_7;

// control pins
const RW: u64 = 1 << 24; // when hi read, when lo write (out)
const SYNC: u64 = 1 << 25; // hi when starting new instruction (out)
const IRQ: u64 = 1 << 26; // hi to request irq (in)
const NMI: u64 = 1 << 27;  // hi to request nmi (in)

// flags
const CARRY: u8 = 1 << 0;
const ZERO: u8 = 1 << 1;
const IRQ_DISABLE: u8 = 1 << 2;
const DECIMAL: u8 = 1 << 3;
const OVERFLOW: u8 = 1 << 6;
const NEGATIVE: u8 = 1 << 7;

pub struct Mos6502 {
  pins: u64,
  a: u8,
  x: u8,
  y: u8,
  s: u8,
  p: u8,
  pc: u16
}

impl fmt::Display for Mos6502 {
  fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "A: {:#04x} X: {:#04x} Y: {:#04x} S: {:#04x} P: {:#04x} ({:#010b}) PC: {:#06x} ADDR: {:#06x} DATA: {:#04x} SYNC: {}", self.a, self.x, self.y, self.s, self.p, self.p, self.pc, self.read_addr(), self.read_data(), self.read_sync())
  }
}

impl Mos6502 {
  pub fn new() -> Mos6502 {
    Mos6502 {
      pins: SYNC,
      a: 0,
      x: 0,
      y: 0,
      s: 0,
      p: ZERO,
      pc: 0
    }
  }

  fn set(&mut self, mask: u64) {
    self.pins &= !mask;
  }

  fn clear(&mut self, mask: u64) {
    self.pins &= !mask;
  }

  pub fn step(&mut self) {
    if self.read_sync() == 1 {
      self.clear(SYNC);
    }
  }

  pub fn read_addr(&self) -> u16 {
    (self.pins & ADDR_MASK) as u16
  }

  pub fn read_data(&self) -> u8 {
    ((self.pins & DATA_MASK) >> 16) as u8
  }

  pub fn set_data(&mut self, data: u8) {
    self.pins = (self.pins & !DATA_MASK) | (data as u64) << 16;
  }

  pub fn read_rw(&self) -> u8 {
    (self.pins >> 24) as u8
  }

  pub fn read_sync(&self) -> u8 {
    (self.pins >> 25) as u8
  }

  pub fn set_irq(&mut self) {
    self.set(IRQ);
  }

  pub fn set_nmi(&mut self) {
    self.set(NMI);
  }
 }
