extern crate rv32im_sim;

use rv32im_sim::Arguments;
use std::env;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();

	let args = Arguments::new(&args).unwrap_or_else(|err| {
		println!("problem parsing arguments: {}", err);
		process::exit(1);
	});

	println!("searching for {}", args.query);
	println!("In file {}", args.filename);

	if let Err(e) = rv32im_sim::read_file(args) {
		println!("Application error: {}", e);
		process::exit(1);
	}
}
