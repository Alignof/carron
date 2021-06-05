mod elf_header;
mod section_header;
mod program_header;

use memmap::Mmap;
use std::fs::File;
use crate::cpu::{get_u16, get_u32};
use elf_header::ElfHeader;
use section_header::SectionHeader;
use program_header::ProgramHeader;


pub struct ElfLoader {
        elf_header: ElfHeader,
        prog_headers: Vec<ProgramHeader>,
        sect_headers: Vec<SectionHeader>,
    pub mem_data: Mmap,
}

impl ElfLoader {
    pub fn try_new(filename: &str) -> std::io::Result<ElfLoader>{
        let file = File::open(filename)?;
        let mapped_data = unsafe{Mmap::map(&file)?};
        let new_elf  = ElfHeader::new(&mapped_data);
        let new_prog = ProgramHeader::new(&mapped_data, &new_elf);
        let new_sect = SectionHeader::new(&mapped_data, &new_elf);

        Ok(ElfLoader{
            elf_header: new_elf,
            prog_headers: new_prog,
            sect_headers: new_sect,
            mem_data: mapped_data,
        })
    }

    pub fn is_elf(&self) -> bool {
        self.elf_header.is_elf()
    }

    pub fn ident_show(&self){
        self.elf_header.ident_show();
    }

    pub fn show_all_header(&self){
        self.elf_header.show();

        println!("\n\n");

        for (id, prog) in self.prog_headers.iter().enumerate(){
            prog.show(id);
        }

        println!("\n\n");

        for (id, sect) in self.sect_headers.iter().enumerate(){
            sect.show(id);
        }

    }

    pub fn dump_segment(&self){
        for (id, prog) in self.prog_headers.iter().enumerate(){
            prog.show(id);
            prog.segment_dump(&self.mem_data);
            println!("\n\n");
        }
    }

    pub fn dump_section(&self){
        for (id, sect) in self.sect_headers.iter().enumerate(){
            if sect.is_dumpable() {
                sect.show(id);
                sect.section_dump(&self.mem_data);
                println!("\n\n");
            }
        }
    }
}

