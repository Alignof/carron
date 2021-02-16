use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub mod elfload;

pub struct Arguments {
	pub filename: String,
}

impl Arguments {
	pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
		if args.len() < 2 {
			return Err("not enough arguments");
		}

		let filename = args[1].clone();

		Ok(Arguments { filename })
	}
}

pub fn read_file(args: Arguments) -> Result<(), Box<dyn Error>> {
	let mut f = File::open(args.filename)?;

	let mut contents = String::new();
	f.read_to_string(&mut contents)?;

	println!("With text:\n {}", contents);

	Ok(())
}
