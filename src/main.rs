mod cpu;
use cpu::cpu::CPU;

fn main() {
    let mut init_registers: [u8; 16] = [0; 16];
    init_registers[0..2].copy_from_slice(&[5, 10]);

    let mut init_memory: [u8; 0x1000] = [0; 0x1000];
    init_memory[0x00..0x06].copy_from_slice(&[0x21, 0x00, 0x21, 0x00, 0x00, 0x00]);
    init_memory[0x100..0x106].copy_from_slice(&[0x80, 0x14, 0x80, 0x14, 0x00, 0xEE]);

    let mut cpu = CPU::new(init_registers, init_memory);
    cpu.run();
}
