pub mod create_mmap;
pub mod write_dtb;

use std::collections::HashMap;

use super::parser::FdtTokenKind;
use super::LabelManager;

pub struct FdtHeader {
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

pub struct DtbMmap {
    reserve: Vec<u64>,
    structure: Vec<u32>,
    strings: Strings,
    labels: LabelManager,
}

impl DtbMmap {
    pub fn new(labels: LabelManager) -> Self {
        DtbMmap {
            reserve: vec![0x0, 0x0],
            structure: Vec::new(),
            strings: Strings::new(),
            labels,
        }
    }

    fn regist_string(&mut self, name: &str) -> u32 {
        let name = format!("{}\0", name);
        let offset_of_name = self.strings.current_offset;
        *self.strings.table.entry(name.clone()).or_insert_with(|| {
            self.strings.current_offset += name.len() as u32;
            offset_of_name
        })
    }

    pub fn label_lookup(&self, label: &str) -> String {
        self.labels.lookup(label).expect("label not found")
    }

    pub fn is_phandle_needed(&self, label_name: &str) -> Option<u32> {
        self.labels.is_phandle_needed(label_name)
    }

    pub fn write_nodekind(&mut self, kind: FdtTokenKind) {
        self.structure.push(kind as u32);
    }

    pub fn write_property(&mut self, name: &str, data: &[u32], size: usize) {
        self.write_nodekind(FdtTokenKind::Prop);
        let offset = self.regist_string(name);
        self.structure.push(size as u32); // data len
        self.structure.push(offset); // prop name offset
        self.structure.extend_from_slice(data);
    }

    pub fn write_nodename(&mut self, name: &str) {
        let name = match name {
            "/" => "",
            _ => name,
        };

        self.structure.append(
            &mut format!("{name}\0")
                .into_bytes()
                .chunks(4)
                .map(|bs| {
                    // &[u8] -> [u8; 4]
                    let mut s = [0; 4];
                    s[..bs.len()].clone_from_slice(bs);
                    u32::from_be_bytes(s)
                })
                .collect(),
        );
    }
}
