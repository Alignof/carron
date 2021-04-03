pub trait Decode {
	fn decode(&self, mmap: &[u8]) -> String;
	fn decode_str(&self, mmap: &[u8]) -> String;
}

impl Decode for SectionHeader {
	fn section_decode(&self, mmap: &[u8]){
		for (block, dump_part) in (self.sh_offset .. self.sh_offset + self.sh_size as u32).step_by(4).enumerate(){
			if block % 16 == 0 { println!() }
			print!("{:08x} ", get_u32(mmap, dump_part as usize));
		}
		println!();
	}
}
