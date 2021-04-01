pub mod elfload;
use memmap::Mmap;
use std::fs::File;
use elfload::{ElfHeader, SectionHeader, ProgramHeader};

pub struct Arguments {
	pub arg_num: usize,
	pub filename: String,
}

impl Arguments {
	pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
		if args.len() < 2 {
			return Err("not enough arguments");
		}

		let arg_num  = args.len();
		let filename = args[1].clone();

		Ok(Arguments { filename, arg_num })
	}
}


pub struct ElfLoader {
	elf_header: ElfHeader,
	prog_headers: Vec<ProgramHeader>,
	sect_headers: Vec<SectionHeader>,
	mem_data: Mmap,
	
}

impl ElfLoader {
	pub fn try_new(filename: &str) -> std::io::Result<ElfLoader>{
		let file = File::open(filename)?;
		let mapped_data = unsafe{Mmap::map(&file)?};
		let new_elf = ElfHeader::new(&mapped_data);
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
		const HEADER_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];
		self.elf_header.e_ident.magic[0..4] == HEADER_MAGIC
	}

	pub fn ident_show(&self){
		self.elf_header.ident_show();
	}

	pub fn show_all_header(&self){
		self.elf_header.show();

		for (id, prog) in self.prog_headers.iter().enumerate(){
			println!("============== program header {}==============", id + 1);
			prog.show();
		}

		for (id, sect) in self.sect_headers.iter().enumerate(){
			println!("============== section header {}==============", id + 1);
			sect.show();
		}

	}

	pub fn dump_segment(&self){
		for (id, prog) in self.prog_headers.iter().enumerate(){
			println!("============== program header {}==============", id + 1);
			prog.show();
			prog.segment_dump(&self.mem_data);
			println!("\n\n");
		}
	}

	pub fn dump_section(&self){
		for (id, sect) in self.sect_headers.iter().enumerate(){
			println!("============== section header {}==============", id + 1);
			sect.show();
			sect.section_dump(&self.mem_data);
			println!("\n\n");
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn elf_header_test() {
		let loader = match ElfLoader::try_new("./src/example_elf") {
			Ok(loader) => loader,
			Err(error) => {
				panic!("There was a problem opening the file: {:?}", error);
			}
		};

		assert!(loader.is_elf());
		assert_eq!(loader.elf_header.e_type, 2);
		assert_eq!(loader.elf_header.e_flags, 5);
		assert_eq!(loader.elf_header.e_version, 1);
		assert_eq!(loader.elf_header.e_machine, 243);
	}

	#[test]
	fn program_header_test() {
		let loader = match ElfLoader::try_new("./src/example_elf") {
			Ok(loader) => loader,
			Err(error) => {
				panic!("There was a problem opening the file: {:?}", error);
			}
		};

		assert_eq!(loader.prog_headers[0].p_type, 1);
		assert_eq!(loader.prog_headers[0].p_flags, 5);
	}
}

