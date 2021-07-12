use std::{u16, usize};

struct CPU {
    registers: [u8; 16],
    program_counter: usize,
    memory: [u8; 0x1000], // 0x1000 == 2^12 in binary => 4096 bytes, that is 4kb
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        let pc = self.program_counter;
        let op_byte1 = self.memory[pc] as u16;
        let op_byte2 = self.memory[pc + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let op_code = self.read_opcode();
            self.program_counter += 2;

            let c = ((op_code & 0xF000) >> 12) as u8;
            let x = ((op_code & 0x0F00) >> 8) as u8;
            let y = ((op_code & 0x00F0) >> 4) as u8;
            let d = ((op_code & 0x000F) >> 0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", op_code),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let first_arg = self.registers[x as usize];
        let second_arg = self.registers[y as usize];

        let (value, overflow) = first_arg.overflowing_add(second_arg);
        self.registers[x as usize] = value;
        match overflow {
            true => self.registers[0xF] = 1,
            false => self.registers[0xF] = 0,
        }
    }
}

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        memory: [0; 4096],
        program_counter: 0,
    };

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    mem[0] = 0x80;
    mem[1] = 0x14;
    mem[2] = 0x80;
    mem[3] = 0x24;
    mem[4] = 0x80;
    mem[5] = 0x34;

    cpu.run();
    assert_eq!(cpu.registers[0], 35);
}
