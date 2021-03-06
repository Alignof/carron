extern crate rv32im_sim;

use std::env;
use std::process;
use rv32im_sim::Arguments;
use rv32im_sim::elfload;

fn main() {
	let args: Vec<String> = env::args().collect();

	let args = Arguments::new(&args).unwrap_or_else(|err| {
		println!("problem parsing arguments: {}", err);
		process::exit(1);
	});

	println!("In file {}", args.filename);

	let loader = match elfload::ElfLoader::try_new(&args.filename){
		Ok(loader) => loader,
		Err(error) => {
			panic!("There was a problem opening the file: {:?}", error);
		},
	};

	if loader.is_elf(){
		println!("elfcheck: OK");
	}

	loader.ident_show();
}
