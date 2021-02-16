use std::fs::File;
use memmap::Mmap;

const HEADER_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

pub struct ElfLoader {
	mapped_file: Mmap,
}

impl ElfLoader {
	pub fn try_new(filename: &str) -> std::io::Result<ElfLoader>{
		let file = File::open(filename)?;
		Ok(ElfLoader{
			mapped_file: unsafe{Mmap::map(&file)?},
		})
	}

	pub fn is_elf(&self) -> bool {
		self.mapped_file[0..4] == HEADER_MAGIC
	}
}

