use super::ElfHeader;
use crate::elfload::{get_u32, get_u16, is_cinst};

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


	pub fn show(&self, id: usize){
		println!("============== program header {}==============", id + 1);
		println!("sh_name:\t{}",	    self.sh_name);
		println!("sh_type:\t{}",	    get_section_type_name(self.sh_type));
		println!("sh_flags:\t{}",	    self.sh_flags);
		println!("sh_addr:\t0x{:x}",	self.sh_addr);
		println!("sh_offset:\t0x{:x}",	self.sh_offset);
		println!("sh_size:\t{}",	    self.sh_size);
		println!("sh_link:\t{}",	    self.sh_link);
		println!("sh_info:\t{}",	    self.sh_info);
		println!("sh_addralign:\t{}",	self.sh_addralign);
		println!("sh_entsize:\t{}",	    self.sh_entsize);
	}

	pub fn section_dump(&self, mmap: &[u8]){
		use crate::decode::Decode;

		println!("--------------------------------");
        let mut dump_head = self.sh_offset;
        while dump_head != self.sh_offset + self.sh_size {
			if is_cinst(mmap, dump_head as usize) {
                let mdump = get_u16(mmap, dump_head as usize);
				let inst  = mdump.decode();
                dump_head += 2;

				print!("{:<04x}\t\t{}\t\t{}", mdump, inst.opc_to_string(), inst.reg_to_string());
                if let Some(v) = inst.rs1 {print!(" {}", v)}
                if let Some(v) = inst.rs2 {print!(" {}", v)}
			}else{
                let mdump = get_u32(mmap, dump_head as usize);
				let inst  = mdump.decode();
                dump_head += 4;

				print!("{:<08x}\t{}\t\t{}", mdump, inst.opc_to_string(), inst.reg_to_string());
                if let Some(v) = inst.rs1 {print!(" {}", v)}
                if let Some(v) = inst.rs2 {print!(" {}", v)}
			}
            println!();
		}
	}

    pub fn is_dumpable(&self) -> bool {
        self.sh_flags >> 2 & 1 == 1
    }
}       
