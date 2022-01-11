extern crate rv32im_sim;
use rv32im_sim::elfload;
use rv32im_sim::Simulator;
use rv32im_sim::system::ExeOption;
use rv32im_sim::system::Arguments;

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
                let mut simulator: Simulator = Simulator::new(loader, args.init_pc); 
                simulator.simulation();
            },
            ExeOption::OPT_ELFHEAD  => loader.header_show(),
            ExeOption::OPT_PROG     => loader.dump_segment(),
            ExeOption::OPT_SECT     => loader.dump_section(),
            ExeOption::OPT_SHOWALL  => loader.show_all_header(),
            ExeOption::OPT_DISASEM  => loader.dump_section(),
        };
    } else {
        panic!("This file is not an ELF.");
    }
}
