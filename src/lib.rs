pub mod elfload;
pub mod decode;

#[allow(non_camel_case_types)]
enum Option {
    OPT_ELFHEAD,
    OPT_PROG,
    OPT_SECT,
    OPT_DISASEM,
    OPT_DEFAULT,
}

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

		Ok(Arguments {
            filename,
            arg_num,
        })
	}
}

