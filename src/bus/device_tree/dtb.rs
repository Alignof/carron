mod parse;

#[allow(non_camel_case_types)]
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

#[allow(non_camel_case_types)]
pub struct dtb_mmap {
    reserve: Vec<u64>,
    structure: Vec<u32>, 
    strings: Vec<u8>,
}

#[allow(non_camel_case_types)]
struct dtb_data {
    header: fdt_header,
    mmap: dtb_mmap,
}

#[allow(non_camel_case_types)]
enum FdtNodeKind {
    BEGIN_NODE = 0x1,
    END_NODE = 0x2,
    PROP = 0x3,
    NOP = 0x4,
    END = 0x9,
}

fn make_dtb(dts: String) -> dtb_data {
    let mut mmap: dtb_mmap = dtb_mmap {
            reserve: vec![0x0, 0x0],
            structure: Vec::new(),
            strings: Vec::new(),
    };
    let mut lines = dts.lines().peekable();

    loop {
        parse::parse_node(&mut lines, &mut mmap);
        if lines.peek().is_none() {
            break;
        }
    }
    mmap.structure.push(FdtNodeKind::END as u32);

    dtb_data {
        header: fdt_header {
            magic: 0xd00dfeed,
            totalsize: 0,
            off_dt_struct: 0x38,
            off_dt_strings: 0,
            off_mem_rsvmap: 0,
            version: 17,
            last_comp_version: 0,
            boot_cpuid_phys: 0,
            size_dt_strings: 0,
            size_dt_struct: 0,
        },
        mmap,
    }
}
