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

#[test]
fn load_and_store_operations(){
    let mut cpu = CPU::new_with_memory(vec![
        0x60, 0x0A,     //  set register 0 to 10
        0x61, 0x08,     //  set register 1 to 8
        0x62, 0x01,     //  set register 2 to 1
        0x63, 0x04,     //  set register 3 to 4
        0xA1, 0x00,     //  set pointer register to 0x100
        0xF3, 0x55,     //  store register from 0 to 3 in memory[pointer_register]
        0xA1, 0x02,     //  set pointer register to 0x102
        0xF1, 0x65,     //  load register from 0 to 1 reading from memory[pointer_register]
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0x0), 1);
    assert_eq!(cpu.peek_register(0x1), 4);
    assert_eq!(cpu.peek_register(0x2), 1);
    assert_eq!(cpu.peek_register(0x3), 4);
}

#[test]
fn add_to_pointer_register(){
    let mut cpu = CPU::new_with_memory(vec![
        0x60, 0x0A,     //  set register 0 to 10
        0x61, 0x08,     //  set register 1 to 8
        0x62, 0x02,     //  set register 2 to 2
        0x63, 0x04,     //  set register 3 to 4
        0xA1, 0x00,     //  set pointer register to 0x100
        0xF3, 0x55,     //  store register from 0 to 3 in memory[pointer_register]
        0xF2, 0x1E,     //  add register 2 to pointer register
        0xF1, 0x65,     //  load register from 0 to 1 reading from memory[pointer_register]
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0x0), 2);
    assert_eq!(cpu.peek_register(0x1), 4);
    assert_eq!(cpu.peek_register(0x2), 2);
    assert_eq!(cpu.peek_register(0x3), 4);
}

#[test]
#[should_panic]
fn illegal_read_through_load(){
    let mut cpu = CPU::new_with_memory(vec![
        0xAF, 0xFF,     //  set pointer register to 0xFFF
        0x60, 0x0A,     //  set register 0 to 10
        0xF0, 0x1E,     //  add register 0 to pointer register
        0xFA, 0x65,     //  load starting from address 4105 (panic)
    ]);
    cpu.run();
}

#[test]
#[should_panic]
fn illegal_write_through_store(){
    let mut cpu = CPU::new_with_memory(vec![
        0xAF, 0xFF,     //  set pointer register to 0xFFF
        0x60, 0x0A,     //  set register 0 to 10
        0xF0, 0x1E,     //  add register 0 to pointer register
        0xFA, 0x55,     //  store starting at address 4105 (panic)
    ]);
    cpu.run();
}

#[test]
fn storing_as_bcd(){
    let mut cpu = CPU::new_with_memory(vec![
        0xA0, 0x0A,     //  set pointer register to 0xFFF
        0x60, 0xC0,     //  set register 0 to 192
        0xF0, 0x33,     //  store register 0 as BCD
        0xF2, 0x65,     //  load in registers up to register 2
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0), 1);
    assert_eq!(cpu.peek_register(1), 9);
    assert_eq!(cpu.peek_register(2), 2);
}