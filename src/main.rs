mod mos6502;

fn main() {
    let cpu = mos6502::Mos6502::new();
    println!("{}", cpu);
}
