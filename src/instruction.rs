use super::cpu::CPU;
use super::register::Reg;

enum Operand {
    Imm8,
    Imm16,
    Register(Reg),
}

struct Instruction {
    code: u16,
    op1: Option<Operand>,
    op2: Option<Operand>,
}

macro_rules! make_instruction {
}

impl Instruction {
    fn length(&self) -> u16 {
        let mut result = match self.code {
            0..=0xFF => 1,
            0xCB00..=0xCBFF => 2,
            code => panic!("opcode 0x{:X} out of range", code),
        };
        for op in [self.op1, self.op2].iter() {
            result += match op {
                Some(Operand::Imm8) => 1,
                Some(Operand::Imm16) => 2,
                _ => 0,
            }
        }
        result
    }

    fn decode(&self, code: &[u8]) -> 
}

struct InstructionSet {
    instructions: Vec<Instruction>,
}

impl InstructionSet {
    fn get_instruction(&self, mut opcode: u16) -> Option<&Instruction> {
        if opcode >> 8 != 0xCB {
            opcode &= 0x00FF;
        }
        match self.instructions.binary_search_by_key(&opcode, |i| i.code) {
            Ok(result) => Some(&self.instructions[result]),
            _ => None,
        }
    }
}
