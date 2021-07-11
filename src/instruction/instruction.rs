use super::super::cpu::CPU;

pub type FnExecute = fn(&mut CPU) -> u8;

struct Instruction {
    remaining_cycles: usize,
    instruction_size: usize,
    has_jumped: bool,
}

impl Instruction {
    fn execute(&mut self) {}
}
