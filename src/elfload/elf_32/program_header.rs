use super::ElfHeader32;
use crate::elfload::{get_u32, ProgramHeader};

fn get_segment_type_name(segment_type: u32) -> &'static str {
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

pub struct ProgramHeader32 {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    p_memsz: u32,
    p_flags: u32,
    p_align: u32,
}

impl ProgramHeader32 {
    pub fn new(mmap: &[u8], elf_header: &ElfHeader32) -> Vec<Box<dyn ProgramHeader>> {
        let mut new_prog: Vec<Box<dyn ProgramHeader>> = Vec::new();

        for segment_num in 0..elf_header.e_phnum {
            let segment_start: usize =
                (elf_header.e_phoff + (elf_header.e_phentsize * segment_num) as u32) as usize;

            new_prog.push(Box::new(ProgramHeader32 {
                p_type: get_u32(mmap, segment_start),
                p_offset: get_u32(mmap, segment_start + 4),
                p_vaddr: get_u32(mmap, segment_start + 8),
                p_paddr: get_u32(mmap, segment_start + 12),
                p_filesz: get_u32(mmap, segment_start + 16),
                p_memsz: get_u32(mmap, segment_start + 20),
                p_flags: get_u32(mmap, segment_start + 24),
                p_align: get_u32(mmap, segment_start + 28),
            }));
        }

        new_prog
    }
}

impl ProgramHeader for ProgramHeader32 {
    fn show(&self, id: usize) {
        println!("============== program header {}==============", id + 1);
        println!("p_type:\t\t{}", get_segment_type_name(self.p_type));
        println!("p_offset:\t0x{:x}", self.p_offset);
        println!("p_vaddr:\t0x{:x}", self.p_vaddr);
        println!("p_paddr:\t0x{:x}", self.p_paddr);
        println!("p_filesz:\t0x{:x}", self.p_filesz);
        println!("p_memsz:\t0x{:x}", self.p_memsz);
        println!("p_flags:\t{}", self.p_flags);
        println!("p_align:\t0x{:x}", self.p_align);
    }

    fn dump(&self, mmap: &[u8]) {
        for (block, dump_part) in (self.p_offset..self.p_offset + self.p_memsz)
            .step_by(4)
            .enumerate()
        {
            if block % 8 == 0 {
                println!()
            }
            print!("{:08x} ", get_u32(mmap, dump_part as usize));
        }
    }

    fn offset_and_addr(&self) -> (u64, u64) {
        (self.p_offset as u64, self.p_paddr as u64)
    }

    fn is_loadable(&self) -> bool {
        self.p_type == 0x1
    }

    fn p_filesz(&self) -> u64 {
        self.p_filesz as u64
    }
}
