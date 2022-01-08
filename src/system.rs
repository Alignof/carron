use clap::{ArgGroup, arg};

#[allow(non_camel_case_types)]
pub enum ExeOption {
    OPT_DEFAULT,
    OPT_ELFHEAD,
    OPT_PROG,
    OPT_SECT,
    OPT_DISASEM,
    OPT_SHOWALL,
}

pub struct Arguments {
    pub filename: String,
    pub exe_option: ExeOption,
    pub init_pc: Option<u32>,
}

impl Arguments {
    pub fn new() -> Arguments {
        let app = clap::app_from_crate!()
            .arg(arg!([filename] "ELF file path").group("ELF"))
            .arg(arg!(--pc <init_pc> ... "entry program counter").required(false))
            .arg(arg!(-p --program ... "see add segments"))
            .arg(arg!(-s --section ... "see all sections"))
            .arg(arg!(-d --disasem ... "disassemble ELF"))
            .arg(arg!(-a --all ... "see all data"))
            .group(
                ArgGroup::new("run option")
                    .args(&["disasem", "program", "section", "all"])
                    .requires("ELF")
                    .required(false)
            )
            .get_matches();

        dbg!(app.value_of("run option"));

        let filename = match app.value_of("filename") {
            Some(f) => f.to_string(),
            None => panic!("please specify target ELF file."),
        };

        let flag_map = | | {
            (
                app.is_present("program"),
                app.is_present("section"),
                app.is_present("disasem"),
                app.is_present("all")
            )
        };
        let exe_option = match flag_map() {
            (true, _, _, _) => ExeOption::OPT_DISASEM,
            (_, true, _, _) => ExeOption::OPT_SECT,
            (_, _, true, _) => ExeOption::OPT_PROG,
            (_, _, _, true) => ExeOption::OPT_SHOWALL,
            _ => ExeOption::OPT_DEFAULT,
        };

        let init_pc = app.value_of("pc")
            .map(|x| x.parse::<u32>().expect("invalid pc"));

        Arguments {
            filename,
            exe_option,
            init_pc,
        }
    }
}
