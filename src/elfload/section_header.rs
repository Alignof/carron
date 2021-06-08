use super::ElfHeader;
use crate::cpu::{get_u32, get_u16, is_cinst};

pub struct SectionHeader {
    sh_name: String,
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
        let name_table = elf_header.e_shoff + (elf_header.e_shentsize * elf_header.e_shstrndx) as u32;
        let name_table_off: usize = get_u32(mmap, (name_table as usize) + 16) as usize;

        for section_num in 0 .. elf_header.e_shnum {
            let section_head: usize = (elf_header.e_shoff + (elf_header.e_shentsize * section_num) as u32) as usize;

            new_sect.push(
                SectionHeader {
                    sh_name:      SectionHeader::get_sh_name(mmap, section_head, name_table_off),
                    sh_type:      get_u32(mmap, section_head +  4),
                    sh_flags:     get_u32(mmap, section_head +  8),
                    sh_addr:      get_u32(mmap, section_head + 12),
                    sh_offset:    get_u32(mmap, section_head + 16),
                    sh_size:      get_u32(mmap, section_head + 20),
                    sh_link:      get_u32(mmap, section_head + 24),
                    sh_info:      get_u32(mmap, section_head + 28),
                    sh_addralign: get_u32(mmap, section_head + 32),
                    sh_entsize:   get_u32(mmap, section_head + 34),
                }       
            );
        }

        return new_sect;
    }

    fn get_sh_name(mmap: &[u8], section_head: usize, name_table_head: usize) -> String {
        let name_id: usize = get_u32(mmap, section_head) as usize;
        let mut sh_name: String = String::new();

        for c in mmap[name_table_head + name_id ..].iter() {
            if *c as char == '\0' {
                break;
            }
            sh_name.push(*c as char);
        }
        return sh_name;
    }

    fn type_to_str(&self) -> &'static str {
        match self.sh_type {
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

    pub fn show(&self, id: usize){
        println!("============== section header {}==============", id + 1);
        println!("sh_name:\t{}",        self.sh_name);
        println!("sh_type:\t{}",        self.type_to_str());
        println!("sh_flags:\t{}",       self.sh_flags);
        println!("sh_addr:\t0x{:x}",    self.sh_addr);
        println!("sh_offset:\t0x{:x}",  self.sh_offset);
        println!("sh_size:\t{}",        self.sh_size);
        println!("sh_link:\t{}",        self.sh_link);
        println!("sh_info:\t{}",        self.sh_info);
        println!("sh_addralign:\t{}",   self.sh_addralign);
        println!("sh_entsize:\t{}",     self.sh_entsize);
    }


    pub fn section_dump(&self, mmap: &[u8]){
        use crate::cpu::decode::Decode;

        println!("--------------------------------");
        let mut dump_head = self.sh_offset;
        while dump_head != self.sh_offset + self.sh_size {
            if is_cinst(mmap, dump_head as usize) {
                let mdump = get_u16(mmap, dump_head as usize);
                let inst  = mdump.decode();
                dump_head += 2;

                print!("{:<04x}\t\t", mdump);
                inst.print_myself();
            }else{
                let mdump = get_u32(mmap, dump_head as usize);
                let inst  = mdump.decode();
                dump_head += 4;

                print!("{:<08x}\t", mdump);
                inst.print_myself();
            }
            println!();
        }
    }

    pub fn is_dumpable(&self) -> bool {
        self.sh_flags >> 2 & 1 == 1
    }
}       
