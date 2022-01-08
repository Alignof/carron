use clap::{ArgGroup, arg};

#[allow(non_camel_case_types)]
pub enum ExeOption {
    OPT_DEFAULT,
    OPT_ELFHEAD,
    OPT_PROG,
    OPT_SECT,
    OPT_SHOWALL,
    OPT_DISASEM,
}

pub struct Arguments {
    pub filename: String,
    pub exe_option: ExeOption,
}

impl Arguments {
    pub fn new() -> Arguments {
        let app = clap::app_from_crate!()
            .arg(arg!([filename] "ELF file").group("ELF"))
            .arg(arg!(--pc <init_pc> ... "entry program counter"))
            .arg(arg!(-d --disasem ... "disassemble ELF"))
            .arg(arg!(-p --program ... "see add segments"))
            .arg(arg!(-s --section ... "see all sections"))
            .arg(arg!(-a --all ... "see all data"))
            .group(
                ArgGroup::new("run option")
                    .args(&["disasem", "program", "section", "all"])
                    .requires("ELF")
                    .required(false)
            )
            .get_matches();

        dbg!(app.value_of("pc"));

        let filename = match app.value_of("filename") {
            Some(f) => f.to_string(),
            None => panic!("please specify target ELF file."),
        };
        let exe_option = if app.is_present("run option") {
            ExeOption::OPT_DEFAULT
        } else {
            ExeOption::OPT_DEFAULT
        };

        Arguments {
            filename,
            exe_option,
        }
    }
}
