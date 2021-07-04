use super::super::cpu::CPU;
use super::instruction::FnExecute;

fn add_instructions(
    single_byte_opcodes: &mut [FnExecute; 256],
    multi_byte_opcodes: &mut [FnExecute; 256],
) {
    single_byte_opcodes[0x21] = load_HL_d16;
}

fn load_HL_d16(cpu: &mut CPU) -> u8 {
    1
}
