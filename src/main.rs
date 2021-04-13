extern crate rv32im_sim;

use rv32im_sim::elfload;
use rv32im_sim::ExeOption;
use rv32im_sim::Arguments;
use std::env;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();

	let args = Arguments::new(&args).unwrap_or_else(|err| {
		println!("problem parsing arguments: {}", err);
		process::exit(1);
	});

	println!("In file {}", args.filename);

	let loader = match elfload::ElfLoader::try_new(&args.filename) {
		Ok(loader) => loader,
		Err(error) => {
			panic!("There was a problem opening the file: {:?}", error);
		}
	};

	if loader.is_elf() {
		println!("elfcheck: OK");
	}

    if matches!(args.exe_option, ExeOption::OPT_ELFHEAD) {
		println!("option -h");
    }

    match args.exe_option {
        ExeOption::OPT_ELFHEAD	=> loader.ident_show(),
        ExeOption::OPT_PROG	    => loader.dump_segment(),
        ExeOption::OPT_SECT	    => loader.dump_section(),
        ExeOption::OPT_SHOWALL	=> loader.show_all_header(),
        ExeOption::OPT_DISASEM	=> loader.ident_show(),
        ExeOption::OPT_DEFAULT	=> loader.ident_show(),
    }
}
