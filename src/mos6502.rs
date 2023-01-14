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

//
const NMI_VECTOR: u16 = 0xFFFA;
const RESET_VECTOR: u16 = 0xFFFC;
const BRK_IRQ_VECTOR: u16 = 0xFFFE;

pub struct Mos6502 {
  pins: u64, // addr, data, and control pins
  a: u8,
  x: u8,
  y: u8,
  s: u8, // stack pointer
  p: u8, // status
  pc: u16, // program counter
  ir: u8, // instruction register
  ic: u8, // instruction count, used to keep track of which cycle we are on in the instruction
  ad: u16 // adh/adl internal address bus
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
      pc: 0,
      ir: 0,
      ic: 0, 
      ad: 0 
    }
  }

  fn set(&mut self, mask: u64) {
    self.pins &= !mask;
  }

  fn clear(&mut self, mask: u64) {
    self.pins &= !mask;
  }

  pub fn read_addr(&self) -> u16 {
    (self.pins & ADDR_MASK) as u16
  }

  fn set_addr(&mut self, data: u16) {
    self.pins = (self.pins & !ADDR_MASK) | data as u64
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

  pub fn step(&mut self) {
    if self.read_sync() == 1 {
      self.ir = self.read_data();
      self.ic = 0;
      self.clear(SYNC);
    }

    self.set(RW); // read by default

    match (self.ir, self.ic) {
      (0x00, 0) => { self.set_addr(self.pc) },
      (0x00, 1) => { 
        if true { self.pc += 1 } // if NOT IRQ or NMI, TODO: impl 
        self.s = self.s.wrapping_sub(1);
        self.set_addr(self.s as u16 | 0x0100);
        self.set_data((self.pc >> 8) as u8); // store pc hi
        if false { self.clear(RW) } // if IRQ or NMI, TODO: impl 
      },
      (0x00, 2) => {
        self.s = self.s.wrapping_sub(1);
        self.set_addr(self.s as u16 | 0x0100);
        self.set_data(self.pc as u8);  // store pc lo
        if false { self.clear(RW) } // if IRQ or NMI, TODO: impl 
      },
      (0x00, 3) => {
        self.s = self.s.wrapping_sub(1);
        self.set_addr(self.s as u16 | 0x0100);
        self.set_data(self.p);  // store p TODO: do I need (| 1 << 5)
        if true {
          self.ad = RESET_VECTOR;
        } else { // if IRQ or NMI, TODO: impl
          // TODO: check and set addr vecotr based on NMI vs IRQ
          self.clear(RW)
        }
      },
      (0x00, 4) => {
        self.set_addr(self.ad);
        self.ad += 1;
      },
      (0x00, 5) => {
        self.set_addr(self.ad);
        self.ad = self.read_data() as u16; // fetch pc lo from vector
      },
      (0x00, 6) => {
        self.pc = (self.read_data() as u16) << 8 | self.ad; // fetch pc hi from vector + 1 and combine with pc lo
        // TODO: fetch next opcode
      }
      _ => panic!("Unhanlded instruction {:#04x}", self.ir)
    };

    self.ic += 1;
  }
 }
