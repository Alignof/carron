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
    pub fn regist_label(&mut self, name: String, addr_cells:u32) {
        self.labels.insert(name, addr_cells);
    }

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
        self.structure.extend_from_slice(data);
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
                    let mut s = [0; 4];
                    s[.. bs.len()].clone_from_slice(bs);
                    u32::from_be_bytes(s)
                })
                .collect()
        );
    }
}

#[allow(non_camel_case_types, dead_code)]
pub enum FdtNodeKind {
    BEGIN_NODE = 0x1,
    END_NODE = 0x2,
    PROP = 0x3,
    NOP = 0x4,
    END = 0x9,
}

pub fn make_dtb(dts: String) -> Vec<u8> {
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

    make_dtb_mmap(mmap)
}

fn make_dtb_mmap(mmap: dtb_mmap) -> Vec<u8> {
    let reserve = mmap.reserve
        .iter()
        .flat_map(|x| x.to_be_bytes())
        .collect::<Vec<u8>>();

    let structure = mmap.structure
        .iter()
        .flat_map(|x| x.to_be_bytes())
        .collect::<Vec<u8>>();

    let strings = mmap.strings.table.keys()
        .cloned()
        .flat_map(|s| s.into_bytes())
        .collect::<Vec<u8>>();

    let size_dt_header = 0x28;
    let size_dt_reserve = util::align_size(reserve.len(), 8);
    let size_dt_strings = util::align_size(strings.len(), 4);
    let size_dt_struct = util::align_size(structure.len(), 4);
    let totalsize = size_dt_header + size_dt_reserve + size_dt_strings + size_dt_struct;

    let header = fdt_header {
        magic: 0xd00dfeed,
        totalsize,
        off_dt_struct: size_dt_header + size_dt_reserve,
        off_dt_strings: size_dt_header + size_dt_reserve + size_dt_struct,
        off_mem_rsvmap: size_dt_header,
        version: 16,
        last_comp_version: 16,
        boot_cpuid_phys: 0,
        size_dt_strings,
        size_dt_struct,
    };


    let mut mmap: Vec<u8> = Vec::new();
    mmap.extend(header.magic.to_be_bytes());
    mmap.extend(header.totalsize.to_be_bytes());
    mmap.extend(header.off_dt_struct.to_be_bytes());
    mmap.extend(header.off_dt_strings.to_be_bytes());
    mmap.extend(header.off_mem_rsvmap.to_be_bytes());
    mmap.extend(header.version.to_be_bytes());
    mmap.extend(header.last_comp_version.to_be_bytes());
    mmap.extend(header.boot_cpuid_phys.to_be_bytes());
    mmap.extend(header.size_dt_strings.to_be_bytes());
    mmap.extend(header.size_dt_struct.to_be_bytes());
    mmap.extend(reserve);
    mmap.extend(structure);
    mmap.extend(strings);

    mmap
}

