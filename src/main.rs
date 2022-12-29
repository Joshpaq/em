use std::fmt;

struct Registers {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    p: u8
}

struct Mos6502 {
    r: Registers,
    pc: u16,
    addr: u16,
    data: u8,
    sync: bool,
    rw: bool // hi read lo write
}

impl fmt::Display for Mos6502 {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A: {:#04x} X: {:#04x} Y: {:#04x} S: {:#04x} P: {:#010b} PC: {:#06x} ADDR: {:#06x} DATA: {:#04x}", self.r.a, self.r.x, self.r.y, self.r.s, self.r.p, self.pc, self.addr, self.data)
    }
}

impl Mos6502 {
    fn new () -> Mos6502 {
        Mos6502 { 
            r: Registers { 
                a: 0, 
                x: 0, 
                y: 0, 
                s: 0, 
                p: 0 
            },
            pc: 0,
            addr: 0,
            data: 0,
            sync: false,
            rw: true
        }
    }
}

fn main() {
    let cpu = Mos6502::new();
    println!("{}", cpu);
}
