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
