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
    dbg!(data);
    let mut data_ch = &mut data.chars();
    match data_ch.next().unwrap() {
        '"' => {
            let str_data: String = data_ch
                .take_while(|c| *c != '"')
                .collect::<String>();
        },
        '<' => {
            let int_data: u32 = data_ch
                .take_while(|c| *c != '>')
                .collect::<String>()
                .parse()
                .expect("parsing integer error.");
        },
        _ => panic!("prop data is invalid"),
    }

    if data_ch.last() != Some(';') {
        panic!("{} <-- ';' expected.", data);
    }
}

pub fn parse_property(lines: &mut Peekable<std::str::Lines>, mmap: &mut dtb_mmap) {
    let mut tokens = util::tokenize(lines, "device tree is invalid");
    let prop_name = tokens.next().expect("prop name not found");

    util::expect(tokens.next(), "=");

    let raw_data = tokens.next().expect("data not found");
    parse_data(raw_data, mmap);
}

pub fn parse_node(lines: &mut Peekable<std::str::Lines>, mmap: &mut dtb_mmap) {
    dbg!(&lines.peek());
    // node
    if lines.peek().unwrap().chars().last() == Some('{') {
        // expect node's name and "{"
        let mut tokens = util::tokenize(lines, "device tree is invalid");
        let node_name = tokens.next().expect("node name not found");
        util::expect(tokens.next(), "{");

        loop {
            parse_node(lines, mmap);

            // expect "};"
            let last_token = lines.peek().unwrap().split(' ').last();
            if last_token == Some("};") {
                break;
            }
        }

    // property
    } else {
        parse_property(lines, mmap);
    }
}


