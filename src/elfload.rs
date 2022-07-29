mod elf_header;
mod section_header;
mod program_header;

use std::fs::File;
use memmap::Mmap;
use elf_header::ElfHeader;
use section_header::SectionHeader;
use program_header::ProgramHeader;


#[allow(clippy::identity_op)]
pub fn get_u16(mmap: &[u8], index: usize) -> u16 {
    (mmap[index + 1] as u16) << 8 |
    (mmap[index + 0] as u16)
}

#[allow(clippy::identity_op)]
pub fn get_u32(mmap: &[u8], index: usize) -> u32 {
    (mmap[index + 3] as u32) << 24 |
    (mmap[index + 2] as u32) << 16 |
    (mmap[index + 1] as u32) <<  8 |
    (mmap[index + 0] as u32)
}

pub fn is_cinst(mmap: &[u8], index: usize) -> bool {
    mmap[index] & 0x3 != 0x3
}

pub struct ElfLoader {
    pub elf_header: ElfHeader,
    pub prog_headers: Vec<ProgramHeader>,
        sect_headers: Vec<SectionHeader>,
    pub mem_data: Mmap,
}

impl ElfLoader {
    pub fn try_new(filename: &str) -> std::io::Result<ElfLoader> {
        let file = File::open(filename)?;
        let mapped_data = unsafe{ Mmap::map(&file)? };
        let new_elf  = ElfHeader::new(&mapped_data);
        let new_prog = ProgramHeader::new(&mapped_data, &new_elf);
        let new_sect = SectionHeader::new(&mapped_data, &new_elf);

        Ok(ElfLoader {
            elf_header: new_elf,
            prog_headers: new_prog,
            sect_headers: new_sect,
            mem_data: mapped_data,
        })
    }

    pub fn is_elf(&self) -> bool {
        self.elf_header.is_elf()
    }

    pub fn get_entry_point(&self) -> Option<u32> {
        for segment in self.prog_headers.iter() {
            if segment.is_loadable() {
                return Some(segment.p_paddr);
            }
        }

        None
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
            prog.segment_dump(&self.mem_data);
            println!("\n\n");
        }
    }

    pub fn dump_section(&self) {
        for (id, sect) in self.sect_headers.iter().enumerate() {
            if sect.is_dumpable() {
                sect.show(id);
                sect.section_dump(&self.mem_data);
                println!("\n\n");
            }
        }
    }
}

