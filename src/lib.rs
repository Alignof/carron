pub mod elfload;
pub mod decode;

#[allow(non_camel_case_types)]
pub enum ExeOption {
    OPT_ELFHEAD,
    OPT_PROG,
    OPT_SECT,
    OPT_SHOWALL,
    OPT_DISASEM,
}

fn parse_option(option: &str) -> Result<ExeOption, &'static str> {
    match option {
        "-h" => Ok(ExeOption::OPT_ELFHEAD),
        "-p" => Ok(ExeOption::OPT_PROG),
        "-s" => Ok(ExeOption::OPT_SECT),
        "-a" => Ok(ExeOption::OPT_SHOWALL),
        "-d" => Ok(ExeOption::OPT_DISASEM),
        _    => Err("invalid option"),
    }
}

pub struct Arguments {
	pub arg_num: usize,
	pub filename: String,
    pub exe_option: ExeOption,
}

impl Arguments {
	pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
		if args.len() < 2 {
			return Err("not enough arguments");
		}

        let arg_num  = args.len();
        let filename: String;
        let exe_option: ExeOption;

        // no option
        if args.len() == 2 {
            exe_option = ExeOption::OPT_DEFAULT;
            filename = args[1].clone();
        } else {
            exe_option = match parse_option(&args[1]) {
                Ok(opt) => opt,
                Err(msg) => panic!("{}", msg),
            };
            filename = args[2].clone();
        }

		Ok(Arguments {
            filename,
            arg_num,
            exe_option,
        })
	}
}

