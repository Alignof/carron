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

#[cfg(test)]
mod tests{
	use super::*;

	#[test]
	fn elfload_test(){
		let loader = match elfload::ElfLoader::try_new("./src/example_elf"){
			Ok(loader) => loader,
			Err(error) => {
				panic!("There was a problem opening the file: {:?}", error);
			},
		};

		assert!(loader.is_elf());
	}
}
