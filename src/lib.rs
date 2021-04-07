pub mod elfload;
pub mod decode;

pub struct Arguments {
	pub arg_num: usize,
	pub filename: String,
}

impl Arguments {
	pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
		if args.len() < 2 {
			return Err("not enough arguments");
		}

		let arg_num  = args.len();
		let filename = args[1].clone();

		Ok(Arguments { filename, arg_num })
	}
}

