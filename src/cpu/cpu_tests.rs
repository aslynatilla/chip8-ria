#[cfg(test)]
use crate::cpu::cpu::CPU;

#[test]
fn rust_in_action_last_example() {
    let mut init_registers: [u8; 16] = [0; 16];
    init_registers[0..2].copy_from_slice(&[5, 10]);

    let mut init_memory = [0u8; 0x1000];
    init_memory[0x00..0x06].copy_from_slice(&[0x21, 0x00, 0x21, 0x00, 0x00, 0x00]);
    init_memory[0x100..0x106].copy_from_slice(&[0x80, 0x14, 0x80, 0x14, 0x00, 0xEE]);

    let mut cpu = CPU::new(init_registers, init_memory);
    cpu.run();
    assert_eq!(cpu.peek_register(0), 45);
}

#[test]
#[should_panic(expected = "Stack overflow")]
fn chip8_stack_overflows() {
    let call_subroutine_command_x16 = vec![0x20, 0x00].repeat(16);
    let mut cpu = CPU::new_with_memory(call_subroutine_command_x16);
    cpu.run();
}

#[test]
#[should_panic(expected = "Stack underflow")]
fn chip8_stack_underflows() {
    let mut cpu = CPU::new_with_memory(vec![0x00, 0xEE]);
    cpu.run();
}

#[test]
fn offset_jump(){
    let mut cpu = CPU::new_with_memory(vec![
        0x70, 0x07,     //  add 0x07 to register 0
        0xB0, 0x01,     //  jump to register 0 plus 0x01
        0xC0, 0xC0,
        0x0F, 0xD0,
        0x70, 0x02      //  add 0x02 to register 0
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0), 9);
}

#[test]
#[should_panic(expected = "illegal address")]
fn illegal_jump(){
    //  Jumping to the last byte of memory shouldn't be allowed.
    let mut cpu = CPU::new_with_memory(vec![0x1F, 0xFF]);
    cpu.run();
}

#[test]
fn load_number_to_register(){
    let mut cpu = CPU::new_with_memory(vec![0x60, 0xFF]);
    cpu.run();
    assert_eq!(cpu.peek_register(0), 0xFF);
}

#[test]
fn skip_instructions_constant_comparison(){
    let mut cpu = CPU::new_with_memory(vec![
        0x60, 0x07,     //  set register 0 to 0x07
        0x30, 0x07,     //  skip if register 0 contains 0x07
        0x70, 0x07,
        0x40, 0x07,     //  skip if register 0 does not contain 0x07
        0x70, 0x03      //  add 0x03 to register 0
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0), 10);
}

#[test]
fn skip_instructions_register_comparison(){
    let mut cpu = CPU::new_with_memory(vec![
        0x60, 0x07,     //  set register 0 to 0x07
        0x61, 0x07,     //  set register 0 to 0x07
        0x50, 0x10,     //  skip if register 0 is equal to register 1
        0x70, 0x07,
        0x90, 0x10,     //  skip if register 0 is not equal to register 1
        0x80, 0x14      //  add register 0 to register 1 and store result in register 0
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0), 14);
    assert_eq!(cpu.peek_register(1), 7);
}

#[test]
fn register_copy(){
    let mut cpu = CPU::new_with_memory(vec![
        0x60, 0x10,     //  set register 0 to 0x10
        0x81, 0x00,     //  copies register 0 to register 1
        0x71, 0x02      //  add 0x02 to register 1
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0), 16);
    assert_eq!(cpu.peek_register(1), 18);
}

#[test]
fn logical_operators(){
    let mut cpu = CPU::new_with_memory(vec![
        0x60, 0b0000_0000,      //  set register 0 to 0x00
        0x61, 0b0000_0110,      //  set register 1 to 0x06
        0x80, 0x11,             //  register 0 | register 1
        0x62, 0b0000_0100,      //  set register 2 to 0x04
        0x81, 0x22,             //  register 1 & register 2
        0x63, 0b0000_1100,      //  set register 3 to 0x0C
        0x82, 0x33              //  register 2 ^ register 3
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0), 6);
    assert_eq!(cpu.peek_register(1), 4);
    assert_eq!(cpu.peek_register(2), 8);
}

#[test]
fn registers_subtraction(){
    let mut cpu = CPU::new_with_memory(vec![
        0x60, 0x0A,     //  set register 0 to 10
        0x61, 0x08,     //  set register 1 to 8
        0x62, 0x01,     //  set register 2 to 1
        0x80, 0x15,     //  subtract register 1 from register 0
        0x82, 0x05,     //  subtract register 0 from register 2
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0x0), 2);
    assert_eq!(cpu.peek_register(0xF), 1);
}

#[test]
fn shifting_registers(){
    let mut cpu = CPU::new_with_memory(vec![
        0x60, 0x00,     //  set register 0 to 10
        0x61, 0x01,     //  set register 1 to 8
        0x80, 0x13,     //  register 0 | register 1
        0x81, 0x0E,     //  shift left register 1
        0x80, 0x06,     //  shift right register 0 (overflow)
        0x62, 0xFF,     //  set register 2 to 255
        0x82, 0x0E,     //  shift left register 2  (overflow)
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0x0), 0);
    assert_eq!(cpu.peek_register(0x1), 2);
    assert_eq!(cpu.peek_register(0x2), 254);
    assert_eq!(cpu.peek_register(0xF), 1);
}

#[test]
fn registers_swap_subtraction(){
    let mut cpu = CPU::new_with_memory(vec![
        0x60, 0x0A,     //  set register 0 to 10
        0x61, 0x08,     //  set register 1 to 8
        0x81, 0x07,     //  subtract register 0 from register 1
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0x1), 2);
    assert_eq!(cpu.peek_register(0xF), 0);
}