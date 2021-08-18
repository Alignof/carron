extern crate rv32im_sim;

use std::env;
use std::process;
use rv32im_sim::elfload;
use rv32im_sim::Simulator;
use rv32im_sim::system::ExeOption;
use rv32im_sim::system::Arguments;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = Arguments::new(&args).unwrap_or_else(|err| {
        println!("problem occured while parsing arguments: {}", err);
        process::exit(1);
    });

    println!("\nIn file {}", args.filename);

    let loader = match elfload::ElfLoader::try_new(&args.filename) {
        Ok(loader) => loader,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };

    if loader.is_elf() {
        println!("elfcheck: OK\n");

        match args.exe_option {
            ExeOption::OPT_NONE     => {
                let mut simulator: Simulator = Simulator::new(loader); 
                simulator.simulation();
            },
            ExeOption::OPT_ELFHEAD  => loader.header_show(),
            ExeOption::OPT_PROG     => loader.dump_segment(),
            ExeOption::OPT_SECT     => loader.dump_section(),
            ExeOption::OPT_SHOWALL  => loader.show_all_header(),
            ExeOption::OPT_DISASEM  => loader.dump_section(),
        }
    } else {
        panic!("This file is not an ELF.");
    }
}
