use super::CPU;

pub trait Execution {
    fn execution(&self, cpu:CPU);
}

impl Execution for Instruction {
    fn execution(&self, cpu:CPU) {}
}
