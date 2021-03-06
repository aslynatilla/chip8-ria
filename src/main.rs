mod cpu;
use cpu::cpu::CPU;

fn main() {
    let mut cpu = CPU::new_with_memory(vec![
        0x12, 0x4E,
        0x08, 0x19,
        0x01, 0x01,
        0x08, 0x01,
        0x0F, 0x01,
        0x01, 0x09,
        0x08, 0x09,
        0x0F, 0x09,
        0x01, 0x11,
        0x08, 0x11,
        0x0F, 0x11,
        0x01, 0x19,
        0x0F, 0x19,
        0x16, 0x01,
        0x16, 0x09,
        0x16, 0x11,
        0x16, 0x19,
        0xFC, 0xFC,
        0xFC, 0xFC,
        0xFC, 0xFC,
        0xFC, 0x00,
        0xA2, 0x02,
        0x82, 0x0E,
        0xF2, 0x1E,
        0x82, 0x06,
        0xF1, 0x65,
        0x00, 0xEE,
        0xA2, 0x02,
        0x82, 0x0E,
        0xF2, 0x1E,
        0x82, 0x06,
        0xF1, 0x55,
        0x00, 0xEE,
        0x6F, 0x10,
        0xFF, 0x15,
        0xFF, 0x07,
        0x3F, 0x00,
        0x12, 0x46,
        0x00, 0xEE,
        0x00, 0xE0,
        0x62, 0x00,
        0x22, 0x2A,
        0xF2, 0x29,
        0xD0, 0x15,
        0x70, 0xFF,
        0x71, 0xFF,
        0x22, 0x36,
        0x72, 0x01,
        0x32, 0x10,
        0x12, 0x52,
        0xF2, 0x0A,
        0x22, 0x2A,
        0xA2, 0x22,
        0xD0, 0x17,
        0x22, 0x42,
        0xD0, 0x17,
        0x12, 0x64,
    ]);
    cpu.run();
}
