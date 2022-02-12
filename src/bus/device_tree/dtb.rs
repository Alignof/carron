mod parse;
mod util;

use std::collections::HashMap;

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

struct Strings {
    pub table: HashMap<String, u32>, // str, offset
    pub current_offset: u32,
}

impl Strings {
    pub fn new() -> Strings {
        Strings {
            table: HashMap::new(),
            current_offset: 0,
        }
    }
}

#[allow(non_camel_case_types)]
pub struct dtb_mmap {
    reserve: Vec<u64>,
    structure: Vec<u32>, 
    strings: Strings,
    labels: HashMap<String, u32>, // label, address-cells 
    current_label: Option<String>,
}

impl dtb_mmap {
    fn regist_string(&mut self, name: &str) -> u32 {
        let offset_of_name = self.strings.current_offset;
        self.strings.table
            .entry(name.to_string())
            .or_insert(self.strings.current_offset);
        self.strings.current_offset += name.len() as u32;

        offset_of_name
    }

    pub fn write_nodekind(&mut self, kind: FdtNodeKind) {
        self.structure.push(kind as u32);
    }

    pub fn write_property(&mut self, name: &str, data: &mut Vec<u32>) {
        let offset = self.regist_string(name);
        self.structure.push(data.len() as u32 * 4); // data len
        self.structure.push(offset); // prop name offset
        self.structure.append(data);
    }

    pub fn write_nodename(&mut self, name: &str) {
        let offset = self.regist_string("node_name");
        self.structure.push(name.len() as u32); // data len
        self.structure.push(offset); // prop name offset
        self.structure.append(
            &mut name
                .to_string()
                .into_bytes()
                .chunks(4)
                .map(|bs| {
                    // &[u8] -> [u8; 4]
                    u32::from_be_bytes(bs.try_into().expect("from_be_bytes failed"))
                })
                .collect()
        );
    }
}

#[allow(non_camel_case_types)]
pub enum FdtNodeKind {
    BEGIN_NODE = 0x1,
    END_NODE = 0x2,
    PROP = 0x3,
    NOP = 0x4,
    END = 0x9,
}

#[allow(non_camel_case_types)]
pub struct dtb_data {
    header: fdt_header,
    mmap: dtb_mmap,
}

pub fn make_dtb(dts: String) -> dtb_data {
    let mut mmap: dtb_mmap = dtb_mmap {
            reserve: vec![0x0, 0x0],
            structure: Vec::new(),
            strings: Strings::new(),
            labels: HashMap::new(),
            current_label: None,
    };
    let mut lines = dts.lines().peekable();

    while lines.peek().is_some() {
        parse::parse_line(&mut lines, &mut mmap);
    }
    mmap.write_nodekind(FdtNodeKind::END);

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
