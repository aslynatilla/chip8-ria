#[cfg(test)]
use crate::cpu::CPU;

#[test]
fn dummy_test() {
    assert!(true)
}

#[test]
fn rust_in_action_last_example() {
    let mut init_registers: [u8; 16] = [0; 16];
    init_registers[0..2].copy_from_slice(&[5, 10]);

    let mut init_memory= [0u8; 0x1000];
    init_memory[0x00..0x06].copy_from_slice(&[0x21, 0x00, 0x21, 0x00, 0x00, 0x00]);
    init_memory[0x100..0x106].copy_from_slice(&[0x80, 0x14, 0x80, 0x14, 0x00, 0xEE]);

    let mut cpu = CPU::new(init_registers, init_memory);
    cpu.run();
    assert_eq!(cpu.registers[0], 45);
}

#[test]
#[should_panic(expected="Stack overflow")]
fn chip8_stack_overflows(){
    let mut init_memory = [0u8; 0x1000];
    let call_subroutine_command_x16 = vec![0x20, 0x00].repeat(16);
    init_memory[0x0000..0x0020].copy_from_slice(call_subroutine_command_x16.as_slice());
    let mut cpu = CPU::new([0; 16], init_memory);
    cpu.run();
}

#[test]
#[should_panic(expected="Stack underflow")]
fn chip8_stack_underflows(){
    let mut init_memory = [0u8; 0x1000];
    init_memory[0x0000..0x0002].copy_from_slice(&[0x00, 0xEE]);
    let mut cpu = CPU::new([0; 16], init_memory);
    cpu.run();
}