mod elf_32;
mod elf_64;

use memmap::Mmap;
use std::fs::File;

use crate::Isa;
use elf_32::elf_header::ElfHeader32;
use elf_32::program_header::ProgramHeader32;
use elf_32::section_header::SectionHeader32;
use elf_64::elf_header::ElfHeader64;
use elf_64::program_header::ProgramHeader64;
use elf_64::section_header::SectionHeader64;

pub fn get_u16(mmap: &[u8], index: usize) -> u16 {
    (mmap[index + 1] as u16) << 8 | (mmap[index] as u16)
}

pub fn get_u32(mmap: &[u8], index: usize) -> u32 {
    (mmap[index + 3] as u32) << 24
        | (mmap[index + 2] as u32) << 16
        | (mmap[index + 1] as u32) << 8
        | (mmap[index] as u32)
}

pub fn get_u64(mmap: &[u8], index: usize) -> u64 {
    (mmap[index + 7] as u64) << 56
        | (mmap[index + 6] as u64) << 48
        | (mmap[index + 5] as u64) << 40
        | (mmap[index + 4] as u64) << 32
        | (mmap[index + 3] as u64) << 24
        | (mmap[index + 2] as u64) << 16
        | (mmap[index + 1] as u64) << 8
        | (mmap[index] as u64)
}

pub fn is_cinst(mmap: &[u8], index: usize) -> bool {
    mmap[index] & 0x3 != 0x3
}

pub struct ElfIdentification {
    magic: [u8; 16],
    class: u8,
    endian: u8,
    version: u8,
    os_abi: u8,
    os_abi_ver: u8,
}

impl ElfIdentification {
    fn new(mmap: &[u8]) -> Self {
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

    fn target_arch(&self) -> Isa {
        const EI_CLASS: usize = 4;
        if self.magic[EI_CLASS] == 1 {
            Isa::Rv32
        } else {
            Isa::Rv64
        }
    }

    fn show(&self) {
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

pub struct ElfLoader {
    pub elf_header: Box<dyn ElfHeader>,
    pub prog_headers: Vec<Box<dyn ProgramHeader>>,
    pub sect_headers: Vec<Box<dyn SectionHeader>>,
    pub mem_data: Mmap,
}

impl ElfLoader {
    #[allow(dead_code)]
    fn addr2offset(prog_headers: &[Box<dyn ProgramHeader>], addr: u64) -> Option<u64> {
        let mut addr_table = Vec::new();
        for seg in prog_headers {
            addr_table.push(seg.offset_and_addr());
        }

        addr_table.sort_by(|x, y| (x.1).cmp(&y.1));
        for w in addr_table.windows(2) {
            let (a, z) = (w[0], w[1]);
            if a.1 <= addr && addr < z.1 {
                return Some(a.0 + (addr - a.1));
            }
        }

        None
    }

    pub fn try_new(filename: &str) -> std::io::Result<ElfLoader> {
        let file = File::open(filename)?;
        let mapped_data = unsafe { Mmap::map(&file)? };
        let elf_ident = ElfIdentification::new(&mapped_data);
        match elf_ident.target_arch() {
            Isa::Rv32 => {
                let new_elf = ElfHeader32::new(&mapped_data, elf_ident);
                let new_prog = ProgramHeader32::new(&mapped_data, &new_elf);
                let new_sect = SectionHeader32::new(&mapped_data, &new_elf);

                Ok(ElfLoader {
                    elf_header: new_elf,
                    prog_headers: new_prog,
                    sect_headers: new_sect,
                    mem_data: mapped_data,
                })
            }
            Isa::Rv64 => {
                let new_elf = ElfHeader64::new(&mapped_data, elf_ident);
                let new_prog = ProgramHeader64::new(&mapped_data, &new_elf);
                let new_sect = SectionHeader64::new(&mapped_data, &new_elf);

                Ok(ElfLoader {
                    elf_header: new_elf,
                    prog_headers: new_prog,
                    sect_headers: new_sect,
                    mem_data: mapped_data,
                })
            }
        }
    }

    pub fn target_arch(&self) -> Isa {
        self.elf_header.target_arch()
    }

    pub fn is_elf(&self) -> bool {
        self.elf_header.is_elf()
    }

    pub fn get_entry_point(&self) -> Option<u64> {
        for segment in self.prog_headers.iter() {
            if segment.is_loadable() {
                let (_, paddr) = segment.offset_and_addr();
                return Some(paddr);
            }
        }

        None
    }

    pub fn get_host_addr(&self, isa: Isa) -> (Option<u64>, Option<u64>) {
        let symtab = self.sect_headers.iter().find(|&s| s.sh_name() == ".symtab");
        let strtab = self.sect_headers.iter().find(|&s| s.sh_name() == ".strtab");

        let mut tohost = None;
        let mut fromhost = None;
        if let (Some(symtab), Some(strtab)) = (symtab, strtab) {
            let st_size: usize = match isa {
                Isa::Rv32 => 16,
                Isa::Rv64 => 24,
            };
            for symtab_off in symtab.section_range().step_by(st_size) {
                let st_name_off = get_u32(&self.mem_data, symtab_off as usize);
                let st_name = &self.mem_data[(strtab.sh_offset() + st_name_off as u64) as usize..]
                    .iter()
                    .take_while(|c| **c as char != '\0')
                    .map(|c| *c as char)
                    .collect::<String>();

                if st_name == "tohost" {
                    tohost = match isa {
                        Isa::Rv32 => Some(u64::from(get_u32(
                            &self.mem_data,
                            (symtab_off + 4) as usize,
                        ))),
                        Isa::Rv64 => Some(get_u64(&self.mem_data, (symtab_off + 4) as usize)),
                    }
                }

                if st_name == "fromhost" {
                    fromhost = match isa {
                        Isa::Rv32 => Some(u64::from(get_u32(
                            &self.mem_data,
                            (symtab_off + 4) as usize,
                        ))),
                        Isa::Rv64 => Some(get_u64(&self.mem_data, (symtab_off + 4) as usize)),
                    }
                }

                if tohost.is_some() && fromhost.is_some() {
                    break;
                }
            }
        }

        (tohost, fromhost)
    }

    pub fn header_show(&self) {
        self.elf_header.show();
    }

    pub fn show_all_header(&self) {
        self.elf_header.show();

        println!("\n\n");

        for (id, prog) in self.prog_headers.iter().enumerate() {
            prog.show(id);
        }

        println!("\n\n");

        for (id, sect) in self.sect_headers.iter().enumerate() {
            sect.show(id);
        }
    }

    pub fn dump_segment(&self) {
        for (id, prog) in self.prog_headers.iter().enumerate() {
            prog.show(id);
            prog.dump(&self.mem_data);
            println!("\n\n");
        }
    }

    pub fn dump_section(&self) {
        for (id, sect) in self.sect_headers.iter().enumerate() {
            if sect.is_dumpable() {
                sect.show(id);
                sect.dump(&self.mem_data);
                println!("\n\n");
            }
        }
    }
}

pub trait ElfHeader {
    fn show(&self);
    fn target_arch(&self) -> Isa;
    fn is_elf(&self) -> bool;
}

pub trait ProgramHeader {
    fn show(&self, id: usize);
    fn dump(&self, mmap: &[u8]);
    fn is_loadable(&self) -> bool;
    fn offset_and_addr(&self) -> (u64, u64);
    fn p_filesz(&self) -> u64;
}

pub trait SectionHeader {
    fn get_sh_name(mmap: &[u8], section_head: usize, name_table_head: usize) -> String
    where
        Self: Sized;
    fn sh_name(&self) -> &str;
    fn sh_offset(&self) -> u64;
    fn section_range(&self) -> std::ops::Range<u64>;
    fn type_to_str(&self) -> &'static str;
    fn show(&self, id: usize);
    fn dump(&self, mmap: &[u8]);
    fn is_dumpable(&self) -> bool;
}
