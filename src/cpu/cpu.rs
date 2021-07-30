#![allow(dead_code)]

use std::ops::{ShlAssign, ShrAssign, BitAnd, Div, Rem};
use rand::Rng;
use ggez::{conf::{self, WindowMode}, event::{self, EventHandler}, graphics::{self}, ContextBuilder, GameResult, Context};
use ggez::graphics::{Color, DrawParam, FilterMode};
use ggez::event::winit_event::{Event, WindowEvent, KeyboardInput, ElementState};
use ggez::input::keyboard;
use ggez::conf::WindowSetup;
use ggez::event::{EventsLoop, KeyCode};

pub struct CPU {
    registers: [u8; 16],
    program_counter: usize,
    memory: [u8; 0x1000],
    stack: [u16; 16],
    stack_pointer: usize,
    pointer_register: u16,

    display: VirtualDisplay<bool>,
    waiting_for_input: bool,
    input_register_index: Option<usize>,
    keycode_map: [KeyCode; 16],
}

#[derive(Clone)]
struct VirtualDisplay<T> {
    data: [T; 2048],
    //  size is 64*32
    dirty_bit: bool,
}

impl EventHandler for VirtualDisplay<bool> {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.dirty_bit {
            let image_bytes = self.data.iter()
                .map(|bit| match bit {
                    true => vec![255u8, 255u8, 255u8, 255u8],
                    false => vec![0u8, 0u8, 0u8, 255u8]
                })
                .flatten()
                .collect::<Vec<u8>>();

            let mut image =
                ggez::graphics::Image::from_rgba8(ctx, 64, 32, &image_bytes)?;
            let draw_params = DrawParam::default().scale([10.0, 10.0]);
            image.set_filter(FilterMode::Nearest);
            ggez::graphics::draw(ctx, &image, draw_params)?;
            self.dirty_bit = false;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

impl CPU {
    pub fn default() -> CPU {
        let mut memory = [0u8; 0x1000];
        let init_memory: [u8; 82] =
            [0x12, 0x00,                     //  Jump to 0x200
                0xF0, 0x90, 0x90, 0x90, 0xF0,   //  Font set starts here
                0x20, 0x60, 0x20, 0x20, 0x70,
                0xF0, 0x10, 0xF0, 0x80, 0xF0,
                0xF0, 0x10, 0xF0, 0x10, 0xF0,
                0x90, 0x90, 0xF0, 0x10, 0x10,
                0xF0, 0x80, 0xF0, 0x10, 0xF0,
                0xF0, 0x80, 0xF0, 0x90, 0xF0,
                0xF0, 0x10, 0x20, 0x40, 0x40,
                0xF0, 0x90, 0xF0, 0x90, 0xF0,
                0xF0, 0x90, 0xF0, 0x10, 0xF0,
                0xF0, 0x90, 0xF0, 0x90, 0x90,
                0xE0, 0x90, 0xE0, 0x90, 0xE0,
                0xF0, 0x80, 0x80, 0x80, 0xF0,
                0xE0, 0x90, 0x90, 0x90, 0xE0,
                0xF0, 0x80, 0xF0, 0x80, 0xF0,
                0xF0, 0x80, 0xF0, 0x80, 0x80
            ];
        memory[0..82].copy_from_slice(&init_memory);
        CPU {
            registers: [0u8; 16],
            program_counter: 0,
            memory,
            stack: [0u16; 16],
            stack_pointer: 0,
            pointer_register: 0,
            display: VirtualDisplay {
                data: [false; 2048],
                dirty_bit: false,
            },
            waiting_for_input: false,
            input_register_index: None,
            keycode_map: [KeyCode::X,
                KeyCode::Key1, KeyCode::Key2, KeyCode::Key3,
                KeyCode::Q, KeyCode::W, KeyCode::E,
                KeyCode::A, KeyCode::S, KeyCode::D,
                KeyCode::Z, KeyCode::C, KeyCode::Key4,
                KeyCode::R, KeyCode::F, KeyCode::V],
        }
    }

    pub fn new(registers: [u8; 16], memory_init: Vec<u8>) -> CPU {
        let mut cpu = CPU::default();
        cpu.registers = registers;
        cpu.memory[0x200..0x200 + memory_init.len()].copy_from_slice(memory_init.as_slice());
        cpu
    }

    //TODO: find a better name for this function
    pub fn new_with_memory(memory_init: Vec<u8>) -> CPU {
        let mut cpu = CPU::default();
        cpu.memory[0x200..0x200 + memory_init.len()].copy_from_slice(memory_init.as_slice());
        cpu
    }

    pub fn run(&mut self) {
        let (mut ctx, mut event_loop) = CPU::create_context_and_loop();

        graphics::clear(&mut ctx, Color::from_rgb(0, 0, 0));

        while ctx.continuing {
            ctx.timer_context.tick();
            event_loop.poll_events(|event| {
                    ctx.process_event(&event);
                    self.handle_event(&mut ctx, event);
                }
            );

            while ggez::timer::check_update_time(&mut ctx, 60) {
                if !self.waiting_for_input {
                    self.emulate_cycle(&mut ctx)
                }
            }

            self.display.draw(&mut ctx).unwrap();
        }
    }

    fn create_context_and_loop() -> (Context, EventsLoop) {
        let configuration = conf::Conf {
            window_mode: WindowMode::default().dimensions(640f32, 320f32),
            window_setup: WindowSetup::default().title("CHIP-8 Emulator").vsync(true),
            ..Default::default()
        };

        ContextBuilder::new("CHIP-8 Emulator", "")
            .conf(configuration)
            .build()
            .unwrap()
    }

    fn emulate_cycle(&mut self, mut ctx: &mut Context) {
        let op_code = self.read_opcode();
        self.program_counter += 2;

        let (c, x, y, d) = decompose_opcode(op_code);
        let nnn = op_code & 0x0FFF;
        let kk = (op_code & 0x00FF) as u8;

        match (c, x, y, d) {
            (0x0, 0x0, 0x0, 0x0) => event::quit(&mut ctx),
            (0x0, 0x0, 0xE, 0x0) => self.clear_display(),
            (0x0, 0x0, 0xE, 0xE) => self.ret(),
            (0x1, _, _, _) => self.jump_to(nnn),
            (0x2, _, _, _) => self.call(nnn),
            (0x3, _, _, _) => self.skip_if_equal(x, kk),
            (0x4, _, _, _) => self.skip_if_different(x, kk),
            (0x5, _, _, 0x0) => self.skip_if_equal_registers(x, y),
            (0x6, _, _, _) => self.load_in_register(x, kk),
            (0x7, _, _, _) => self.add_constant(x, kk),
            (0x8, _, _, 0x0) => self.copy_second_to_first(x, y),
            (0x8, _, _, 0x1) => self.or(x, y),
            (0x8, _, _, 0x2) => self.and(x, y),
            (0x8, _, _, 0x3) => self.xor(x, y),
            (0x8, _, _, 0x4) => self.add_registers(x, y),
            (0x8, _, _, 0x5) => self.sub_registers(x, y),
            (0x8, _, _, 0x6) => self.shift_right(x),
            (0x8, _, _, 0x7) => self.sub_registers_swapped(x, y),
            (0x8, _, _, 0xE) => self.shift_left(x),
            (0x9, _, _, 0x0) => self.skip_if_different_registers(x, y),
            (0xA, _, _, _) => self.set_pointer_register(nnn),
            (0xB, _, _, _) => self.offset_jump_to(nnn),
            (0xC, _, _, _) => self.random_and_constant_in(x, kk),
            (0xD, _, _, _) => self.draw_at(x, y, d),
            (0xF, _, 0x0, 0xA) => self.wait_and_store_key_in(x),
            (0xF, _, 0x1, 0xE) => self.add_to_pointer_register(x),
            (0xF, _, 0x2, 0x9) => self.point_to_font_char(x),
            (0xF, _, 0x3, 0x3) => self.store_as_bcd(x),
            (0xF, _, 0x5, 0x5) => self.store_registers_up_to(x),
            (0xF, _, 0x6, 0x5) => self.load_registers_up_to(x),

            _ => todo!("opcode {:04x}", op_code),
        }
    }

    fn handle_event(&mut self, mut ctx: &mut Context, event: Event) {
        let state = &mut self.display;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    if !state.quit_event(&mut ctx) {
                        ggez::event::quit(&mut ctx);
                    }
                }

                WindowEvent::KeyboardInput {
                    input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(current_keycode),
                        modifiers,
                        ..
                    },
                    ..
                } => {
                    let repeat = keyboard::is_key_repeated(&mut ctx);
                    state.key_down_event(&mut ctx, current_keycode, modifiers.into(), repeat);

                    if self.waiting_for_input {
                        match self.keycode_map.iter().enumerate().find(|&(_, &map_keycode)| {
                            map_keycode == current_keycode
                        }) {
                            None => (),
                            Some((value, _)) => {
                                self.registers[self.input_register_index.unwrap()] = value as u8;
                                self.waiting_for_input = false;
                                self.input_register_index = None;
                            }
                        };
                    }
                }

                _ => (),
            }
            _ => (),
        }
    }

    pub(in super) fn peek_register(&self, register_index: usize) -> u8 {
        self.registers[register_index]
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
        self.registers[x as usize] = self.registers[x as usize].wrapping_add(kk);
    }

    fn call(&mut self, fn_address: u16) {
        let (stack_ptr, stack) = (self.stack_pointer, &mut self.stack);

        if stack_ptr >= stack.len() {
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

    fn jump_to(&mut self, address: u16) {
        if !self.is_legal_address(address as usize) {
            panic!("Jumping to illegal address!")
        }
        self.program_counter = address as usize;
    }

    fn is_legal_address(&self, address: usize) -> bool {
        address < self.memory.len() - 1
    }

    fn set_pointer_register(&mut self, address: u16) {
        self.pointer_register = address;
    }

    fn offset_jump_to(&mut self, address: u16) {
        let destination = address + self.registers[0] as u16;
        if !self.is_legal_address(destination as usize) {
            panic!("Jumping to illegal address!")
        }
        self.jump_to(destination);
    }

    fn load_in_register(&mut self, register_index: u8, register_value: u8) {
        let index = register_index as usize;
        if index >= self.registers.len() {
            panic!("Register index out of bounds!")
        }
        self.registers[index] = register_value;
    }

    fn skip_if_equal(&mut self, register_index: u8, comparison_value: u8) {
        if self.registers[register_index as usize] == comparison_value {
            self.program_counter += 2;
        }
    }

    fn skip_if_different(&mut self, register_index: u8, comparison_value: u8) {
        if self.registers[register_index as usize] != comparison_value {
            self.program_counter += 2;
        }
    }

    fn skip_if_equal_registers(&mut self, first_index: u8, second_index: u8) {
        if self.registers[first_index as usize] == self.registers[second_index as usize] {
            self.program_counter += 2;
        }
    }

    fn skip_if_different_registers(&mut self, first_index: u8, second_index: u8) {
        if self.registers[first_index as usize] != self.registers[second_index as usize] {
            self.program_counter += 2;
        }
    }

    fn copy_second_to_first(&mut self, first_index: u8, second_index: u8) {
        self.registers[first_index as usize] = self.registers[second_index as usize];
    }

    fn or(&mut self, first_index: u8, second_index: u8) {
        let (first, second) = (first_index as usize, second_index as usize);
        self.registers[first] |= self.registers[second];
    }

    fn and(&mut self, first_index: u8, second_index: u8) {
        let (first, second) = (first_index as usize, second_index as usize);
        self.registers[first] &= self.registers[second];
    }

    fn xor(&mut self, first_index: u8, second_index: u8) {
        let (first, second) = (first_index as usize, second_index as usize);
        self.registers[first] ^= self.registers[second];
    }

    fn sub_registers(&mut self, first_index: u8, second_index: u8) {
        let (first, second) = (first_index as usize, second_index as usize);
        let (result, overflowing) = self.registers[first].overflowing_sub(self.registers[second]);
        self.registers[first] = result;
        match overflowing {
            true => self.registers[0xF] = 1,
            false => self.registers[0xF] = 0,
        }
    }

    fn shift_right(&mut self, first_index: u8) {
        let first = first_index as usize;
        let first_register = self.registers[first];
        self.registers[0xF] = first_register & 0b0000_0001;
        self.registers[first].shr_assign(1);
    }

    fn shift_left(&mut self, first_index: u8) {
        let first = first_index as usize;
        let first_register = self.registers[first];
        self.registers[0xF] = (first_register & 0b1000_0000) >> 7;
        self.registers[first].shl_assign(1);
    }

    fn sub_registers_swapped(&mut self, first_index: u8, second_index: u8) {
        let (first, second) = (first_index as usize, second_index as usize);
        let (result, overflowing) = self.registers[second].overflowing_sub(self.registers[first]);
        self.registers[first] = result;
        match overflowing {
            true => self.registers[0xF] = 1,
            false => self.registers[0xF] = 0,
        }
    }

    fn random_and_constant_in(&mut self, register_index: u8, constant: u8) {
        let random_num: u8 = rand::thread_rng().gen();
        self.registers[register_index as usize] = random_num.bitand(constant);
    }

    fn store_registers_up_to(&mut self, register_index: u8) {
        let index = register_index as usize;
        let start_address = self.pointer_register as usize;
        let end_address = start_address + index;
        if self.memory.len() < end_address {
            panic!("Writing out of memory bounds.");
        }
        self.memory[start_address..=end_address].copy_from_slice(&self.registers[0..=index]);
    }

    fn load_registers_up_to(&mut self, register_index: u8) {
        let index = register_index as usize;
        let start_address = self.pointer_register as usize;
        let end_address = start_address + index;
        if self.memory.len() < end_address {
            panic!("Illegal read.");
        }
        self.registers[0..=index].copy_from_slice(&self.memory[start_address..=end_address]);
    }

    fn add_to_pointer_register(&mut self, register_index: u8) {
        self.pointer_register += self.registers[register_index as usize] as u16;
    }

    fn store_as_bcd(&mut self, register_index: u8) {
        let i = self.pointer_register as usize;
        let value = self.registers[register_index as usize];
        //  Note that u8 can't represent four-digit numbers, so there is no
        //  need to compute: value % 1000
        self.memory[i + 0] = value / 100u8;
        self.memory[i + 1] = (value % 100u8) / 10u8;
        self.memory[i + 2] = value % 10u8;
    }

    fn point_to_font_char(&mut self, register_index: u8) {
        let char = self.registers[register_index as usize];
        if char.div(16u8) > 0 {    //  if it is a number between 0 and 15
            panic!("Unrepresentable character!");
        }
        self.pointer_register = 2 + 5 * char as u16;
    }

    fn draw_at(&mut self, first_index: u8, second_index: u8, byte_number: u8) {
        let x_coord = self.registers[first_index as usize].rem(64u8);
        let y_coord = self.registers[second_index as usize].rem(32u8);
        self.registers[0xF] = 0;
        let ptr_register = self.pointer_register as usize;
        let sprite = &self.memory[ptr_register..ptr_register + (byte_number as usize)];

        'rows: for i in 0..byte_number {
            if y_coord + i > 32 {
                break 'rows;
            }
            let byte = sprite[i as usize];
            'cols: for j in 0..8 {
                if x_coord + j > 64 {
                    break 'cols;
                }
                let byte = byte.reverse_bits() as u8;
                let current_bit = ((byte >> j) & 0x01) != 0;
                let display_row = (y_coord + i) as usize;
                let display_column = (x_coord + j) as usize;
                let display_index = 64 * display_row + display_column;
                let current_display_bit = self.display.data[display_index];
                self.display.data[display_index as usize] = current_display_bit ^ current_bit;
                self.registers[0xF] = (current_display_bit & current_bit) as u8;
            }
        }

        self.display.dirty_bit = true;
    }

    fn clear_display(&mut self) {
        self.display.data = [false; 2048];
    }

    fn wait_and_store_key_in(&mut self, register_index: u8) {
        self.waiting_for_input = true;
        self.input_register_index = Some(register_index as usize);
    }
}

fn decompose_opcode(op_code: u16) -> (u8, u8, u8, u8) {
    let c = ((op_code & 0xF000) >> 12) as u8;
    let x = ((op_code & 0x0F00) >> 8) as u8;
    let y = ((op_code & 0x00F0) >> 4) as u8;
    let d = ((op_code & 0x000F) >> 0) as u8;
    (c, x, y, d)
}
