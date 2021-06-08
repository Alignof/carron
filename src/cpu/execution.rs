use super::CPU;

pub trait Execution {
    fn execution(&self, cpu:CPU);
}
