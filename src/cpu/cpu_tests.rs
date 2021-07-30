#[cfg(test)]
use crate::cpu::cpu::CPU;

#[test]
fn rust_in_action_last_example() {
    let mut init_registers: [u8; 16] = [0; 16];
    init_registers[0..2].copy_from_slice(&[5, 10]);

    let mut init_memory = [0u8; 0x106];
    init_memory[0x00..0x06].copy_from_slice(&[
        0x23, 0x00,                                 //  Call routine at memory address 0x300
        0x23, 0x00,                                 //  Note that 0x300 = 0x200 + 0x100
        0x00, 0x00]);                               //  Terminate
    init_memory[0x100..0x106].copy_from_slice(&[
        0x80, 0x14,                                 //  Add R1 to R0, twice, and return
        0x80, 0x14,
        0x00, 0xEE
    ]);

    let mut cpu = CPU::new(init_registers, init_memory.to_vec());
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
        0x70, 0x07,     //  Add 0x07 to R0
        0xB2, 0x01,     //  Jump to R0 plus 0x201
        0xC0, 0xC0,
        0x0F, 0xD0,
        0x70, 0x02      //  Add 0x02 to R0
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
        0x60, 0x07,     //  Set R0 to 0x07
        0x30, 0x07,     //  Skip if R0 contains 0x07
        0x70, 0x07,
        0x40, 0x07,     //  Skip if R0 does not contain 0x07
        0x70, 0x03      //  Add 0x03 to R0
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0), 10);
}

#[test]
fn skip_instructions_register_comparison(){
    let mut cpu = CPU::new_with_memory(vec![
        0x60, 0x07,     //  Set R0 to 0x07
        0x61, 0x07,     //  Set R0 to 0x07
        0x50, 0x10,     //  Skip if R0 is equal to R1
        0x70, 0x07,
        0x90, 0x10,     //  Skip if R0 is not equal to R1
        0x80, 0x14      //  Add R0 to R1 and store result in R0
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0), 14);
    assert_eq!(cpu.peek_register(1), 7);
}

#[test]
fn register_copy(){
    let mut cpu = CPU::new_with_memory(vec![
        0x60, 0x10,     //  Set R0 to 0x10
        0x81, 0x00,     //  Copies R0 to R1
        0x71, 0x02      //  Add 0x02 to R1
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0), 16);
    assert_eq!(cpu.peek_register(1), 18);
}

#[test]
fn logical_operators(){
    let mut cpu = CPU::new_with_memory(vec![
        0x60, 0b0000_0000,      //  Set R0 to 0x00
        0x61, 0b0000_0110,      //  Set R1 to 0x06
        0x80, 0x11,             //  R0 | R1
        0x62, 0b0000_0100,      //  Set R2 to 0x04
        0x81, 0x22,             //  R1 & R2
        0x63, 0b0000_1100,      //  Set R3 to 0x0C
        0x82, 0x33              //  R2 ^ R3
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0), 6);
    assert_eq!(cpu.peek_register(1), 4);
    assert_eq!(cpu.peek_register(2), 8);
}

#[test]
fn registers_subtraction(){
    let mut cpu = CPU::new_with_memory(vec![
        0x60, 0x0A,     //  Set R0 to 10
        0x61, 0x08,     //  Set R1 to 8
        0x62, 0x01,     //  Set R2 to 1
        0x80, 0x15,     //  Subtract R1 from R0
        0x82, 0x05,     //  Subtract R0 from R2
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0x0), 2);
    assert_eq!(cpu.peek_register(0xF), 1);
}

#[test]
fn shifting_registers(){
    let mut cpu = CPU::new_with_memory(vec![
        0x60, 0x00,     //  Set R0 to 10
        0x61, 0x01,     //  Set R1 to 8
        0x80, 0x13,     //  R0 | R1
        0x81, 0x0E,     //  Shift left R1
        0x80, 0x06,     //  Shift right R0 (overflow)
        0x62, 0xFF,     //  Set R2 to 255
        0x82, 0x0E,     //  Shift left R2  (overflow)
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
        0x60, 0x0A,     //  Set R0 to 10
        0x61, 0x08,     //  Set R1 to 8
        0x62, 0x01,     //  Set R2 to 1
        0x63, 0x04,     //  Set R3 to 4
        0xA3, 0x00,     //  Set pointer register to 0x300
        0xF3, 0x55,     //  Store register from 0 to 3 in memory[pointer_register]
        0xA3, 0x02,     //  Set pointer register to 0x302
        0xF1, 0x65,     //  Load register from 0 to 1 reading from memory[pointer_register]
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
        0x60, 0x0A,     //  Set R0 to 10
        0x61, 0x08,     //  Set R1 to 8
        0x62, 0x02,     //  Set R2 to 2
        0x63, 0x04,     //  Set R3 to 4
        0xA3, 0x00,     //  Set pointer register to 0x300
        0xF3, 0x55,     //  Store register from 0 to 3 in memory[pointer_register]
        0xF2, 0x1E,     //  Add R2 to pointer register
        0xF1, 0x65,     //  Load register from 0 to 1 reading from memory[pointer_register]
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
        0xAF, 0xFF,     //  Set pointer register to 0xFFF
        0x60, 0x0A,     //  Set R0 to 10
        0xF0, 0x1E,     //  Add R0 to pointer register
        0xFA, 0x65,     //  Load starting from address 4105 (panic)
    ]);
    cpu.run();
}

#[test]
#[should_panic]
fn illegal_write_through_store(){
    let mut cpu = CPU::new_with_memory(vec![
        0xAF, 0xFF,     //  Set pointer register to 0xFFF
        0x60, 0x0A,     //  Set R0 to 10
        0xF0, 0x1E,     //  Add R0 to pointer register
        0xFA, 0x55,     //  Store starting at address 4105 (panic)
    ]);
    cpu.run();
}

#[test]
fn storing_as_bcd(){
    let mut cpu = CPU::new_with_memory(vec![
        0xA0, 0x0A,     //  Set pointer register to 0xFFF
        0x60, 0xC0,     //  Set R0 to 192
        0xF0, 0x33,     //  Store R0 as BCD
        0xF2, 0x65,     //  Load in registers up to R2
    ]);
    cpu.run();
    assert_eq!(cpu.peek_register(0), 1);
    assert_eq!(cpu.peek_register(1), 9);
    assert_eq!(cpu.peek_register(2), 2);
}