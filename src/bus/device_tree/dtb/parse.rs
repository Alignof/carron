use std::iter::Peekable;
use super::util;
use super::dtb_mmap;

#[allow(non_camel_case_types)]
pub enum FdtNodeKind {
    BEGIN_NODE = 0x1,
    END_NODE = 0x2,
    PROP = 0x3,
    NOP = 0x4,
    END = 0x9,
}

pub fn parse_data(data: &str, mmap: &mut dtb_mmap) {
    let mut data_ch = &mut data.chars();
    match data_ch.next().unwrap() {
        '"' => {
            let str_data: String = data_ch
                .skip_while(|c| *c == '"')
                .collect::<String>();
            util::expect(data_ch.next(), '"');
        },
        '<' => {
            let int_data: u32 = data_ch
                .skip_while(|c| *c == '>')
                .collect::<String>()
                .parse()
                .expect("parsing integer error.");
            util::expect(data_ch.next(), '>');
        },
        _ => panic!("prop data is invalid"),
    }

    if data_ch.last() != Some(';') {
        panic!("{} <-- ';' expected.", data);
    }
}

pub fn parse_property(lines: &mut Peekable<std::str::Lines>, mmap: &mut dtb_mmap) {
    let mut tokens = lines.next().expect("device tree is invalid").split(' ');
    let prop_name = tokens.next().expect("prop name not found");

    util::expect(tokens.next(), "=");

    let raw_data = tokens.next().expect("data not found");
    parse_data(raw_data, mmap);
}

pub fn parse_node(lines: &mut Peekable<std::str::Lines>, mmap: &mut dtb_mmap) {
    let mut tokens = lines.next().expect("device tree is invalid").split(' ');

    // expect node's name and "{"
    let node_name = tokens.next().expect("node name not found");
    util::expect(tokens.next(), "{");

    parse_property(lines, mmap);

    // expect "};"
    let mut tokens = lines.next().expect("device tree is invalid").split(' ');
    util::expect(tokens.next(), "};");
}


