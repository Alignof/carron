use clap::arg;

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
            .arg(arg!([filename] "filename of ELF"))
            .arg(arg!(-d --disasem ...))
            .arg(arg!(-p --program ...))
            .arg(arg!(-s --section ...))
            .arg(arg!(-a --all ...));

        Ok(Arguments {
            filename: "/opt/riscv32/share/riscv-tests/isa/rv32ui-v-simple".to_string(),
            exe_option: ExeOption::OPT_ELFHEAD,
        })
    }
}
