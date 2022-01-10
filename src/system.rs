use clap::{AppSettings, ArgGroup, arg};

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
    pub pkpath: Option<String>,
    pub init_pc: Option<u32>,
}

impl Arguments {
    pub fn new() -> Arguments {
        let app = clap::app_from_crate!()
            .arg(arg!(<filename> "ELF file path").group("ELF"))
            .arg(arg!(-e --elfhead ... "Show ELF header"))
            .arg(arg!(-p --program ... "Show all segments"))
            .arg(arg!(-s --section ... "Show all sections"))
            .arg(arg!(-d --disasem ... "Disassemble ELF"))
            .arg(arg!(-a --all ... "Show all ELF data"))
            .group(
                ArgGroup::new("run option")
                    .args(&["elfhead", "disasem", "program", "section", "all"])
                    .required(false)
            )
            .arg(arg!(--pk <proxy_kernel> "Run with proxy kernel").required(false))
            .arg(arg!(--pc <init_pc> ... "Set entry address as hex").required(false))
            .setting(AppSettings::DeriveDisplayOrder)
            .get_matches();

        let filename = match app.value_of("filename") {
            Some(f) => f.to_string(),
            None => panic!("please specify target ELF file."),
        };

        let pkpath = app.value_of("pk").map(|s| s.to_string());

        let flag_map = | | {
            (
                app.is_present("elfhead"),
                app.is_present("program"),
                app.is_present("section"),
                app.is_present("disasem"),
                app.is_present("all")
            )
        };
        let exe_option = match flag_map() {
            (true, _, _, _, _) => ExeOption::OPT_ELFHEAD,
            (_, true, _, _, _) => ExeOption::OPT_DISASEM,
            (_, _, true, _, _) => ExeOption::OPT_SECT,
            (_, _, _, true, _) => ExeOption::OPT_PROG,
            (_, _, _, _, true) => ExeOption::OPT_SHOWALL,
            _ => ExeOption::OPT_DEFAULT,
        };

        let init_pc = app.value_of("pc")
            .map(|x| {
                u32::from_str_radix(x.trim_start_matches("0x"), 16)
                    .expect("invalid pc\nplease set value as hex (e.g. --pc=0x80000000)")
            });

        Arguments {
            filename,
            exe_option,
            pkpath,
            init_pc,
        }
    }
}
