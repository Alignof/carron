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

pub fn parse_property(lines: &mut Peekable<std::str::Lines>, mmap: &mut dtb_mmap) {
}

pub fn parse_node(lines: &mut Peekable<std::str::Lines>, mmap: &mut dtb_mmap) {
    let mut tokens = lines.next().expect("device tree is invalid").split(' ');

    // expect node's name and "{"
    let node_name = tokens.next().expect("node name not found");
    util::consume(tokens.next(), "{");

    parse_property(lines, mmap);

    // expect "};"
    let mut tokens = lines.next().expect("device tree is invalid").split(' ');
    util::consume(tokens.next(), "};");
}


