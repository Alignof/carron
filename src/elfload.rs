use std::fs::File;
use memmap::Mmap;

struct ElfIdentification {
	magic: [u8; 16],
	class: u8,
	endian: u8,
	version: u8,
	os_abi: u8,
	os_abi_ver: u8,
}

impl ElfIdentification {
	fn new(mmap: &[u8]) -> ElfIdentification {
		let mut magic: [u8; 16] = [0; 16];
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
		}
	}

	fn show(&self){
		print!("magic:\t");
		for byte in self.magic.iter() {
			print!("{:02x} ", byte);
		}
		println!();
		println!("class:\t\t{:?}", self.class);
		println!("endian:\t\t{:?}", self.endian);
		println!("version:\t{:?}", self.version);
		println!("os_abi:\t\t{:?}", self.os_abi);
		println!("os_abi_ver:\t{:?}", self.os_abi_ver);
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
		(mmap[index + 1] as u16) << 8 |
		(mmap[index + 0] as u16)
	}

	fn get_u32(mmap: &[u8], index: usize) -> u32 {
		(mmap[index + 3] as u32) << 24 |
		(mmap[index + 2] as u32) << 16 |
		(mmap[index + 1] as u32) <<  8 |
		(mmap[index + 0] as u32)
	}

	fn new(mmap: &[u8]) -> ElfHeader {
		ElfHeader {
			e_ident: ElfIdentification::new(&mmap),
			e_type: ElfHeader::get_u16(mmap, 16),
			e_machine: ElfHeader::get_u16(mmap, 18),
			e_version: ElfHeader::get_u32(mmap, 20),
			e_entry: ElfHeader::get_u32(mmap, 24),
			e_phoff: ElfHeader::get_u32(mmap, 28),
			e_shoff: ElfHeader::get_u32(mmap, 32),
			e_flags: ElfHeader::get_u32(mmap, 36),
			e_ehsize: ElfHeader::get_u16(mmap, 40),
			e_phentsize: ElfHeader::get_u16(mmap, 42),
			e_phnum: ElfHeader::get_u16(mmap, 44),
			e_shentsize: ElfHeader::get_u16(mmap, 46),
			e_shnum: ElfHeader::get_u16(mmap, 48),
			e_shstrndx: ElfHeader::get_u16(mmap, 50),
		}
	}
			
	fn show(&self){
		self.e_ident.show();
		println!("e_type:\t\t{:?}", self.e_type);
		println!("e_machine:\t{:?}", self.e_machine);
		println!("e_version:\t0x{:x?}", self.e_version);
		println!("e_entry:\t0x{:x?}", self.e_entry);
		println!("e_phoff:\t{:?} (bytes into file)", self.e_phoff);
		println!("e_shoff:\t{:?} (bytes into file)", self.e_shoff);
		println!("e_flags:\t0x{:x?}", self.e_flags);
		println!("e_ehsize:\t{:?} (bytes)", self.e_ehsize);
		println!("e_phentsize:\t{:?} (bytes)", self.e_phentsize);
		println!("e_phnum:\t{:?}", self.e_phnum);
		println!("e_shentsize:\t{:?} (bytes)", self.e_shentsize);
		println!("e_shnum:\t{:?}", self.e_shnum);
		println!("e_shstrndx:\t{:?}", self.e_shstrndx);
	}

	fn ident_show(&self){
		self.e_ident.show();
	}
}


pub struct ElfLoader {
	elf_header: ElfHeader,
}

impl ElfLoader {
	pub fn try_new(filename: &str) -> std::io::Result<ElfLoader>{
		let file = File::open(filename)?;
		let mapped_data = unsafe{Mmap::map(&file)?};
		Ok(ElfLoader{
			elf_header: ElfHeader::new(&mapped_data),
		})
	}

	pub fn is_elf(&self) -> bool {
		const HEADER_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];
		self.elf_header.e_ident.magic[0..4] == HEADER_MAGIC
	}

	pub fn ident_show(&self){
		self.elf_header.ident_show();
	}

	pub fn show(&self){
		self.elf_header.show();
	}
}

