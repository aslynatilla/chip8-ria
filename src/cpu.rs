pub struct CPU {
    pub registers: [u8; 16],
    pub program_counter: usize,
    pub memory: [u8; 0x1000],
    pub stack: [u16; 16],
    pub stack_pointer: usize,
}

impl CPU {
    pub fn new(registers : [u8; 16], memory : [u8; 0x1000]) -> CPU {
        CPU {
            registers,
            memory,
            program_counter: 0,
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    pub fn run(&mut self) {
        'running: loop {
            let op_code = self.read_opcode();
            self.program_counter += 2;

            let (c, x, y, d) = decompose_opcode(op_code);
            let nnn = op_code & 0x0FFF;
            let kk = (op_code & 0x00FF) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => break 'running,
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                (0x8, _, _, 0x4) => self.add_registers(x, y),
                (0x7, _, _, _) => self.add_constant(x, kk),

                _ => todo!("opcode {:04x}", op_code),
            }
        }
    }

    fn read_opcode(&self) -> u16 {
        let pc = self.program_counter;
        let op_byte1 = self.memory[pc] as u16;
        let op_byte2 = self.memory[pc + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    fn add_registers(&mut self, x: u8, y: u8) {
        let first_arg = self.registers[x as usize];
        let second_arg = self.registers[y as usize];

        let (value, overflow) = first_arg.overflowing_add(second_arg);
        self.registers[x as usize] = value;
        match overflow {
            true => self.registers[0xF] = 1,
            false => self.registers[0xF] = 0,
        }
    }

    fn add_constant(&mut self, x: u8, kk: u8) {
        self.registers[x as usize] += kk;
    }

    fn call(&mut self, fn_address: u16) {
        let (stack_ptr, stack) = (self.stack_pointer, &mut self.stack);

        if stack_ptr > stack.len() {
            panic!("Stack overflow!");
        }

        stack[stack_ptr] = self.program_counter as u16;
        self.stack_pointer += 1;
        self.program_counter = fn_address as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!");
        }

        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer] as usize;
    }
}

fn decompose_opcode(op_code: u16) -> (u8, u8, u8, u8) {
    let c = ((op_code & 0xF000) >> 12) as u8;
    let x = ((op_code & 0x0F00) >> 8) as u8;
    let y = ((op_code & 0x00F0) >> 4) as u8;
    let d = ((op_code & 0x000F) >> 0) as u8;
    (c, x, y, d)
}
