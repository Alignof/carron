use std::fs::File;
use std::error::Error;
use std::io::prelude::*;

pub struct Arguments{
	pub query: String,
	pub filename: String,
}

impl Arguments{
	pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
		if args.len() < 3{
			return Err("not enough arguments");
		} 

		let query = args[1].clone();
		let filename = args[2].clone();

		Ok(Arguments{query, filename})
	}
}

pub fn read_file(args: Arguments) -> Result<(), Box<dyn Error>> {
	let mut f = File::open(args.filename)?;

	let mut contents = String::new();
	f.read_to_string(&mut contents)?;

	println!("With text:\n {}", contents);

	Ok(())
}


