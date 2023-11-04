use mos6502::{CPU, MEM};

pub mod mos6502;

fn main() {
    let mut mem: MEM = MEM::default();
    let mut cpu: CPU = CPU::default();
    cpu.reset(&mut mem);
    mem.data[0xFFFC] = CPU::INS_JSR;
    mem.data[0xFFFD] = 0x42;
    mem.data[0xFFFE] = 0x42;
    mem.data[0x4242] = CPU::INS_LDA_IM;
    mem.data[0x4243] = 0x12;
    cpu.execute(8,&mut mem);

    println!();

}
