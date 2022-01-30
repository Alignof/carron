use std::iter::Peekable;
use super::{FdtNodeKind, dtb_mmap};

pub fn parse_node(lines: &mut Peekable<std::str::Lines>, mmap: &mut dtb_mmap) {
    let line = lines.next().expect("device tree is invalid");
    match line.chars().last().unwrap() {
        '{' => {
            mmap.structure.push(FdtNodeKind::BEGIN_NODE as u32);
        },
        ';' => {
            mmap.structure.push(FdtNodeKind::PROP as u32);
        },
        _ => panic!("dtb parse error!"),
    };
}


