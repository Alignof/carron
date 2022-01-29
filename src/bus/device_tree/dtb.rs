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

struct fdt_reserve_entry {
    address: u64,
    size: u64,
}

struct fdt_prop {
    len: u32,
    nameoff: u32,
}

struct fdt_node {
    name: String,
    props: Vec<fdt_prop>,
}

struct dtb_data<'a> {
    header: fdt_header,
    reserve: Vec<fdt_reserve_entry>,
    structure: Vec<fdt_node>, 
    strings: Vec<(u32, &'a str)>,
}

fn make_dtb(dts: String) -> dtb_data {
}
