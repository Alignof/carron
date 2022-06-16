extern crate carron;
use carron::elfload;
use carron::Emulator;
use carron::system::ExeOption;
use carron::system::Arguments;

fn main() {
    let args = Arguments::new();

    println!("\nIn file {}", args.filename);

    let loader = match elfload::ElfLoader::try_new(&args.filename) {
        Ok(loader) => loader,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };

    let pk_load = args.pkpath.map(|path| match elfload::ElfLoader::try_new(&path) {
        Ok(pk) => pk,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    });

    if loader.is_elf() {
        println!("elfcheck: OK\n");

        match args.exe_option {
            ExeOption::OPT_DEFAULT  => {
                let mut emulator: Emulator = Emulator::new(
                    loader, pk_load, args.init_pc, args.bpoint, args.rreg,
                ); 
                emulator.emulation();
            },
            ExeOption::OPT_ELFHEAD  => loader.header_show(),
            ExeOption::OPT_DISASEM  => loader.dump_section(),
            ExeOption::OPT_SECT     => loader.dump_section(),
            ExeOption::OPT_PROG     => loader.dump_segment(),
            ExeOption::OPT_SHOWALL  => loader.show_all_header(),
        };
    } else {
        panic!("This file is not an ELF.");
    }
}
