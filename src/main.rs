mod mos6502;

fn main() {
    let mut cpu = mos6502::Mos6502::new();
    println!("{}", cpu);
    cpu.set_data(8);
    println!("{}", cpu);
    cpu.step();
    println!("{}", cpu);
}
