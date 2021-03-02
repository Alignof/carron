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
		println!("magic: {:x?}", self.magic);
		println!("class: {:?}", self.class);
		println!("endian: {:?}", self.endian);
		println!("version: {:?}", self.version);
		println!("os_abi: {:?}", self.os_abi);
		println!("os_abi_ver: {:?}", self.os_abi_ver);
	}
}




struct ElfHeader {
	e_ident: ElfIdentification,
	e_type: u16,
	e_machine: u16,
	e_version: u32,
	e_entry: u32,
	e_phoff: u32,
	e_shoff: u32,
	e_flags: u32,
	e_ehsize: u16,
	e_phentsize: u16,
	e_phnum: u16,
	e_shentsize: u16,
	e_shnum: u16,
	e_shstrndx: u16,
}

impl ElfHeader {
	fn get_u16(mmap: &[u8], index: usize) -> u16 {
		(mmap[index] << 8 + mmap[index + 1]) as u16
	}

	fn get_u32(mmap: &[u8], index: usize) -> u32 {
		(mmap[index] << 24 + mmap[index + 1] << 16 + mmap[index + 2] << 8 + mmap[index + 3]) as u32
	}

	fn new(mmap: &[u8]) -> ElfHeader {
		ElfHeader {
			e_ident: ElfIdentification::new(&mmap),
			e_type: get_u16(mmap, 16),
			e_machine: get_u16(mmap, ),
			e_version: get_u32(mmap, ),
			e_entry: get_u32(mmap, ),
			e_phoff: get_u32(mmap, ),
			e_shoff: get_u32(mmap, ),
			e_flags: get_u32(mmap, ),
			e_ehsize: get_u16(mmap, ),
			e_phentsize: get_u16(mmap, ),
			e_phnum: get_u16(mmap, ),
			e_shentsize: get_u16(mmap, ),
			e_shnum: get_u16(mmap, ),
			e_shstrndx: get_u16(mmap, ),
		}
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
		self.elf_ident.magic[0..4] == HEADER_MAGIC
	}

	pub fn ident_show(&self){
		self.elf_ident.show();
	}
}

