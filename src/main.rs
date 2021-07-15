mod cpu;
use cpu::cpu::CPU;

fn main() {
    let mut cpu = CPU::new_with_memory(vec![
        0x60, 0b0000_0000,      //  set register 0 to 0x00
        0x61, 0b0000_0110,      //  set register 1 to 0x06
        0x80, 0x11,             //  register 0 | register 1
        0x62, 0b0000_0100,      //  set register 2 to 0x04
        0x81, 0x22,             //  register 1 & register 2
        0x63, 0b0000_1100,      //  set register 3 to 0x0C
        0x82, 0x33,              //  register 2 ^ register 3
        0x00, 0xE0,             //  clear display
    ]);
    cpu.run();
}
