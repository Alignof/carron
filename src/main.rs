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

    // initialize a simulator
    let mut simulator: Simulator = match Simulator::try_new(&args.filename) {
        Ok(simulator) => simulator,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    }; 

    if simulator.loader.is_elf() {
        println!("elfcheck: OK\n");

        match args.exe_option {
            ExeOption::OPT_NONE     => simulator.simulation(),
            ExeOption::OPT_ELFHEAD  => simulator.loader.ident_show(),
            ExeOption::OPT_PROG     => simulator.loader.dump_segment(),
            ExeOption::OPT_SECT     => simulator.loader.dump_section(),
            ExeOption::OPT_SHOWALL  => simulator.loader.show_all_header(),
            ExeOption::OPT_DISASEM  => simulator.loader.ident_show(),
        }
    } else {
        panic!("This file is not an ELF.");
    }
}
