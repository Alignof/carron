use std::fs::File;
use memmap::Mmap;

const HEADER_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];

struct ElfIdentification {
	magic: [u8;4],
	class: u8,
	endian: u8,
	version: u8,
	os_abi: u8,
	os_abi_ver: u8,
	reserved: [u8; 7],
}

impl ElfIdentification {
	fn new(mmap: &[u8]) -> ElfIdentification {
		let mut magic: [u8; 4] = [0; 4];
		for (i, m) in mmap[0..4].iter().enumerate() {
			magic[i] = *m;
		}

		ElfIdentification {
			magic,
			class: mmap[4],
			endian: mmap[5],
			version: mmap[6],
			os_abi: mmap[7],
			os_abi_ver: mmap[8],
			reserved: [0; 7],
		}
	}

	fn show(&self){
		println!("magic: {:?}", self.magic)
	}
}

pub struct ElfLoader {
	elf_ident: ElfIdentification,
	mem_mapped: Mmap,
}

impl ElfLoader {
	pub fn try_new(filename: &str) -> std::io::Result<ElfLoader>{
		let file = File::open(filename)?;
		let mapped_data = unsafe{Mmap::map(&file)?};
		Ok(ElfLoader{
			elf_ident:  ElfIdentification::new(&mapped_data),
			mem_mapped: mapped_data,
		})
	}

	pub fn is_elf(&self) -> bool {
		self.mem_mapped[0..4] == HEADER_MAGIC
	}

	pub fn ident_show(&self){
		self.elf_ident.show();
	}
}

