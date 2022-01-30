struct fdt_header {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

struct dtb_data {
    header: fdt_header,
    reserve: Vec<u64>,
    structure: Vec<u32>, 
    strings: Vec<u8>,
}

fn make_dtb(dts: String) -> dtb_data {
    for line in dts.split('\n') {
        match line.chars().last().unwrap() {
            '{' => ,
            ';' => {
                for token in line.split(' ')
            },
        }
    }
}
