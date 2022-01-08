use clap::{ArgGroup, arg};

#[allow(non_camel_case_types)]
pub enum ExeOption {
    OPT_NONE,
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
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        let app = clap::app_from_crate!()
            .arg(arg!([filename] "ELF file").group("ELF"))
            .arg(arg!(-d --disasem ... "disassemble ELF").requires("ELF"))
            .arg(arg!(-p --program ... "see add segments").requires("ELF"))
            .arg(arg!(-s --section ... "see all sections").requires("ELF"))
            .arg(arg!(-a --all ... "see all data").requires("ELF"))
            .group(
                ArgGroup::new("run option")
                    .required(false)
                    .args(&["disasem", "program", "section", "all"])
            )
            .get_matches();

        dbg!(app);

        Ok(Arguments {
            filename: "/opt/riscv32/share/riscv-tests/isa/rv32ui-v-simple".to_string(),
            exe_option: ExeOption::OPT_ELFHEAD,
        })
    }
}
