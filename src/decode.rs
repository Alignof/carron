struct Instruction {
	op: OpecodeKind,
}

pub trait Decode {
	fn decode(&self, mmap: &[u8]) -> Instruction;
}
