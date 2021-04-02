pub trait Decode {
	fn decode(&self, mmap: &[u8]) -> String;
	fn decode_str(&self, mmap: &[u8]) -> String;
}
