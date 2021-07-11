use super::super::cpu::Loc;
use super::super::cpu::Storage;
use super::super::cpu::CPU;
use super::super::decode_instr;
use super::instruction::FnExecute;

fn add_instructions(
    single_byte_opcodes: &mut [FnExecute; 256],
    multi_byte_opcodes: &mut [FnExecute; 256],
) {
    // single_byte_opcodes[0x21] = decode_instr!(Loc::RegHL, imm16, load_16);
}

fn load_16(cpu: &mut CPU, dst: &mut Storage, src: u16) {
    // dst.write_u16(0)
}
