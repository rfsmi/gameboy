use super::super::cpu::CPU;

pub type FnExecute = fn(&mut CPU) -> u8;
