use super::CPU;
use super::instruction::Instruction;

pub trait Execution {
    fn execution(&self, cpu: &CPU);
}

impl Execution for Instruction {
    fn execution(&self, cpu: &CPU) {}
}
