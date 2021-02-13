use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let args: Vec<String> = env::args().collect();

	let args = Arguments::new(&args);
	println!("searching for {}", args.query);
	println!("In file {}", args.filename);

	let mut f = File::open(args.filename).expect("file not found");

	let mut contents = String::new();
	f.read_to_string(&mut contents)
		.expect("something went wrong reading the file");
	println!("With text:\n {}", contents);
}

struct Arguments{
	query: String,
	filename: String,
}

impl Arguments{
	fn new(args: &[String]) -> Arguments {
		let query = args[1].clone();
		let filename = args[2].clone();

		Arguments{query, filename}
	}
}

