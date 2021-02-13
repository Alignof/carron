use std::env;
use std::process;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

fn main() {
	let args: Vec<String> = env::args().collect();

	let args = Arguments::new(&args).unwrap_or_else(|err| {
		println!("problem parsing arguments: {}", err);
		process::exit(1);
	});

	println!("searching for {}", args.query);
	println!("In file {}", args.filename);

	read_file(args);
}

fn read_file(args: Arguments) -> Result<(), Box<Error>>{
	let mut f = File::open(args.filename)?;

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");
	println!("With text:\n {}", contents);

	Ok(())
}

struct Arguments{
	query: String,
	filename: String,
}

impl Arguments{
	fn new(args: &[String]) -> Result<Arguments, &'static str> {
		if args.len() < 3{
			return Err("not enough arguments");
		} 

		let query = args[1].clone();
		let filename = args[2].clone();

		Ok(Arguments{query, filename})
	}
}

