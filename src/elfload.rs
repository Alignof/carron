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

fn get_elf_type_name(elf_type:u16) -> &'static str {
	match elf_type {
		0 => "ET_NONE",
		1 => "ET_REL",
		2 => "ET_EXEC",
		3 => "ET_DYN",
		4 => "ET_CORE",
		_ => "unknown type",
	}
}

fn get_segment_type_name(segment_type:u32) -> &'static str {
	match segment_type {
		0 => "PT_NULL",
		1 => "PT_LOAD",
		2 => "PT_DYNAMIC",
		3 => "PT_INTERP",
		4 => "PT_NOTE",
		5 => "PT_SHLIB",
		6 => "PT_PHDR",
		7 => "PT_LOPROC",
		8 => "PT_HIPROC",
		_ => "unknown type",
	}
}

fn get_section_type_name(section_type:u32) -> &'static str {
	match section_type {
		0  => "SHT_NULL",
		1  => "SHT_PROGBITS",
		2  => "SHT_SYMTAB",
		3  => "SHT_STRTAB",
		4  => "SHT_RELA",
		5  => "SHT_HASH",
		6  => "SHT_DYNAMIC",
		7  => "SHT_NOTE",
		8  => "SHT_NOBITS",
		9  => "SHT_REL",
		10 => "SHT_SHLIB",
		11 => "SHT_DYNSYM",
		12 => "SHT_LOPROC",
		13 => "SHT_HIPROC",
		14 => "SHT_LOUSER",
		15 => "SHT_HIUSER",
		_  => "unknown type",
	}
}

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
		for (i, m) in mmap[0..16].iter().enumerate() {
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


pub struct ElfHeader {
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
	pub fn new(mmap: &[u8]) -> ElfHeader {
		const ELF_HEADER_START: usize = 16;
		ElfHeader {
			e_ident:	ElfIdentification::new(mmap),
			e_type:		get_u16(mmap, ELF_HEADER_START +  0),
			e_machine:	get_u16(mmap, ELF_HEADER_START +  2),
			e_version:	get_u32(mmap, ELF_HEADER_START +  4),
			e_entry:	get_u32(mmap, ELF_HEADER_START +  8),
			e_phoff:	get_u32(mmap, ELF_HEADER_START + 12),
			e_shoff:	get_u32(mmap, ELF_HEADER_START + 16),
			e_flags:	get_u32(mmap, ELF_HEADER_START + 20),
			e_ehsize:	get_u16(mmap, ELF_HEADER_START + 24),
			e_phentsize:	get_u16(mmap, ELF_HEADER_START + 26),
			e_phnum:	get_u16(mmap, ELF_HEADER_START + 28),
			e_shentsize:	get_u16(mmap, ELF_HEADER_START + 30),
			e_shnum:	get_u16(mmap, ELF_HEADER_START + 32),
			e_shstrndx:	get_u16(mmap, ELF_HEADER_START + 34),
		}
	}
			
	pub fn show(&self){
		println!("================ elf header ================");
		self.e_ident.show();
		println!("e_type:\t\t{}",			get_elf_type_name(self.e_type));
		println!("e_machine:\t{}",			self.e_machine);
		println!("e_version:\t0x{:x}",			self.e_version);
		println!("e_entry:\t0x{:x?}",			self.e_entry);
		println!("e_phoff:\t{} (bytes into file)",	self.e_phoff);
		println!("e_shoff:\t{} (bytes into file)",	self.e_shoff);
		println!("e_flags:\t0x{:x}",			self.e_flags);
		println!("e_ehsize:\t{} (bytes)",		self.e_ehsize);
		println!("e_phentsize:\t{} (bytes)",		self.e_phentsize);
		println!("e_phnum:\t{}",			self.e_phnum);
		println!("e_shentsize:\t{} (bytes)",		self.e_shentsize);
		println!("e_shnum:\t{}",			self.e_shnum);
		println!("e_shstrndx:\t{}",			self.e_shstrndx);
	}

	pub fn ident_show(&self){
		self.e_ident.show();
	}
}



pub struct ProgramHeader {
	p_type: u32,
	p_offset: u32,
	p_vaddr: u32,
	p_paddr: u32,
	p_filesz: u32,
	p_memsz: u32,
	p_flags: u32,
	p_align: u32,
}

impl ProgramHeader {
	pub fn new(mmap: &[u8], elf_header:&ElfHeader) -> Vec<ProgramHeader> {
		let mut new_prog = Vec::new();

		for segment_num in 0 .. elf_header.e_phnum {
			let segment_start:usize = (elf_header.e_phoff + (elf_header.e_phentsize * segment_num) as u32) as usize;
			new_prog.push(
				ProgramHeader {
					p_type:   get_u32(mmap, segment_start +  0),
					p_offset: get_u32(mmap, segment_start +  4),
					p_vaddr:  get_u32(mmap, segment_start +  8),
					p_paddr:  get_u32(mmap, segment_start + 12),
					p_filesz: get_u32(mmap, segment_start + 16),
					p_memsz:  get_u32(mmap, segment_start + 20),
					p_flags:  get_u32(mmap, segment_start + 24),
					p_align:  get_u32(mmap, segment_start + 28),
				}
			);
		}

		return new_prog;
	}

	pub fn show(&self){
		println!("p_type:\t\t{}",	get_segment_type_name(self.p_type));
		println!("p_offset:\t0x{:x}",	self.p_offset);
		println!("p_vaddr:\t0x{:x}",	self.p_vaddr);
		println!("p_paddr:\t0x{:x}",	self.p_paddr);
		println!("p_filesz:\t0x{:x}",	self.p_filesz);
		println!("p_memsz:\t0x{:x}",	self.p_memsz);
		println!("p_flags:\t{}",	self.p_flags);
		println!("p_align:\t0x{:x}",	self.p_align);
	}

	pub fn segment_dump(&self, mmap: &[u8]){
		for (block, dump_part) in (self.p_offset .. self.p_offset + self.p_memsz as u32).step_by(4).enumerate(){
			if block % 16 == 0 { println!() }
			print!("{:08x} ", get_u32(mmap, dump_part as usize));
		}
	}
}



pub struct SectionHeader {
	sh_name: u32,
	sh_type: u32,
	sh_flags: u32,
	sh_addr: u32,
	sh_offset: u32,
	sh_size: u32,
	sh_link: u32,
	sh_info: u32,
	sh_addralign: u32,
	sh_entsize: u32,
}
	

impl SectionHeader {
	pub fn new(mmap: &[u8], elf_header:&ElfHeader) -> Vec<SectionHeader> {
		let mut new_sect = Vec::new();

		for section_num in 0 .. elf_header.e_shnum {
			let section_start:usize = (elf_header.e_shoff + (elf_header.e_shentsize * section_num) as u32) as usize;
			new_sect.push(
				SectionHeader {
					sh_name:      get_u32(mmap, section_start +  0),
					sh_type:      get_u32(mmap, section_start +  4),
					sh_flags:     get_u32(mmap, section_start +  8),
					sh_addr:      get_u32(mmap, section_start + 12),
					sh_offset:    get_u32(mmap, section_start + 16),
					sh_size:      get_u32(mmap, section_start + 20),
					sh_link:      get_u32(mmap, section_start + 24),
					sh_info:      get_u32(mmap, section_start + 28),
					sh_addralign: get_u32(mmap, section_start + 32),
					sh_entsize:   get_u32(mmap, section_start + 34),
				}       
			);
		}

		return new_sect;
	}


	pub fn show(&self){
		println!("sh_name:\t{}",	self.sh_name);
		println!("sh_type:\t{}",	get_section_type_name(self.sh_type));
		println!("sh_flags:\t{}",	self.sh_flags);
		println!("sh_addr:\t0x{:x}",	self.sh_addr);
		println!("sh_offset:\t0x{:x}",	self.sh_offset);
		println!("sh_size:\t{}",	self.sh_size);
		println!("sh_link:\t{}",	self.sh_link);
		println!("sh_info:\t{}",	self.sh_info);
		println!("sh_addralign:\t{}",	self.sh_addralign);
		println!("sh_entsize:\t{}",	self.sh_entsize);
	}

	pub fn section_dump(&self, mmap: &[u8]){
		for (block, dump_part) in (self.sh_offset .. self.sh_offset + self.sh_size as u32).step_by(4).enumerate(){
			if block % 16 == 0 { println!() }
			print!("{:08x} ", get_u32(mmap, dump_part as usize));
		}
		println!();
	}
}       


