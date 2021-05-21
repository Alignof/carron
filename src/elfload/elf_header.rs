use crate::elfload::{get_u16, get_u32};

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
    pub e_phoff: u32,
    pub e_shoff: u32,
        e_flags: u32,
        e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

impl ElfHeader {
    pub fn new(mmap: &[u8]) -> ElfHeader {
        const ELF_HEADER_START: usize = 16;
        ElfHeader {
            e_ident:    ElfIdentification::new(mmap),
            e_type:     get_u16(mmap, ELF_HEADER_START +  0),
            e_machine:  get_u16(mmap, ELF_HEADER_START +  2),
            e_version:  get_u32(mmap, ELF_HEADER_START +  4),
            e_entry:    get_u32(mmap, ELF_HEADER_START +  8),
            e_phoff:    get_u32(mmap, ELF_HEADER_START + 12),
            e_shoff:    get_u32(mmap, ELF_HEADER_START + 16),
            e_flags:    get_u32(mmap, ELF_HEADER_START + 20),
            e_ehsize:   get_u16(mmap, ELF_HEADER_START + 24),
            e_phentsize:get_u16(mmap, ELF_HEADER_START + 26),
            e_phnum:    get_u16(mmap, ELF_HEADER_START + 28),
            e_shentsize:get_u16(mmap, ELF_HEADER_START + 30),
            e_shnum:    get_u16(mmap, ELF_HEADER_START + 32),
            e_shstrndx: get_u16(mmap, ELF_HEADER_START + 34),
        }
    }
            
    pub fn show(&self){
        println!("================ elf header ================");
        self.e_ident.show();
        println!("e_type:\t\t{}",                   get_elf_type_name(self.e_type));
        println!("e_machine:\t{}",                  self.e_machine);
        println!("e_version:\t0x{:x}",              self.e_version);
        println!("e_entry:\t0x{:x?}",               self.e_entry);
        println!("e_phoff:\t{} (bytes into file)",  self.e_phoff);
        println!("e_shoff:\t{} (bytes into file)",  self.e_shoff);
        println!("e_flags:\t0x{:x}",                self.e_flags);
        println!("e_ehsize:\t{} (bytes)",           self.e_ehsize);
        println!("e_phentsize:\t{} (bytes)",        self.e_phentsize);
        println!("e_phnum:\t{}",                    self.e_phnum);
        println!("e_shentsize:\t{} (bytes)",        self.e_shentsize);
        println!("e_shnum:\t{}",                    self.e_shnum);
        println!("e_shstrndx:\t{}",                 self.e_shstrndx);
    }

    pub fn ident_show(&self){
        self.e_ident.show();
    }

    pub fn is_elf(&self) -> bool{
        const HEADER_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];
        self.e_ident.magic[0..4] == HEADER_MAGIC
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

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
}

