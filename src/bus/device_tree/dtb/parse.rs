use std::iter::Peekable;
use super::util;
use super::{dtb_mmap, FdtNodeKind};

pub fn parse_data(data: &str, mmap: &mut dtb_mmap) -> (Vec<u32>, u32) {
    if !data.ends_with(';') {
        panic!("{} <-- ';' expected.", data);
    }

    let data_ch = &mut data.chars().peekable();
    match data_ch.next().unwrap() {
        '"' => {
            let mut str_data = String::new();
            loop {
                str_data = format!("{}{}\0", str_data, data_ch
                    .take_while(|c| *c != '"')
                    .collect::<String>());
                if util::consume(data_ch, ',') {
                    util::consume(data_ch, ' ');
                    util::expect(data_ch, '"');
                } else {
                    break;
                }
            }

            let size = str_data.len() as u32;
            let bin = str_data
                .into_bytes()
                .chunks(4)
                .map(|bs| {
                    // &[u8] -> [u8; 4]
                    let mut s = [0; 4];
                    s[.. bs.len()].clone_from_slice(bs);
                    u32::from_be_bytes(s)
                })
                .collect::<Vec<u32>>();

            (bin, size)
        },
        '<' => {
            let bin = data_ch
                .take_while(|c| *c != '>')
                .collect::<String>()
                .split(' ')
                .map(|num| {
                    if let Some(hex) = num.strip_prefix("0x") {
                        u32::from_str_radix(hex, 16).expect("parsing hex error.")
                    } else {
                        num.parse::<u32>().unwrap_or_else(|_| {
                            *mmap.labels
                                .get(num.trim_start_matches('&'))
                                .expect("label referencing error.")
                        })
                    }
                })
                .collect::<Vec<u32>>();

            let size = bin.len() as u32 * 4;
            (bin, size)
        },
        _ => panic!("prop data is invalid"),
    }
}

pub fn parse_property(lines: &mut Peekable<std::str::Lines>, mmap: &mut dtb_mmap) {
    let tokens = &mut util::tokenize(lines, "property is invalid").peekable();
    let prop_name = tokens.next().expect("prop name not found");

    if util::consume(tokens, "=") {
        let raw_data = tokens.collect::<Vec<_>>().join(" ");
        let (mut data_map, data_size) = parse_data(&raw_data, mmap);
        mmap.write_property(prop_name, &mut data_map, data_size);

        if prop_name == "#interrupt-cells" {
            if let Some(inter_cells) = mmap.current_label.clone() {
                mmap.regist_label(inter_cells, data_map[0]);
            }
        }
    } else {
        // only property's name
        let prop_name = prop_name.trim_end_matches(';');
        mmap.write_property(prop_name, &mut Vec::new(), 0);
        if prop_name == "interrupt-controller" {
            mmap.strings.phandle_needed = true;
        }
    }
}

pub fn parse_node(lines: &mut Peekable<std::str::Lines>, mmap: &mut dtb_mmap) {
    // expect node's name and "{"
    let tokens = &mut util::tokenize(lines, "node is invalid").peekable();

    let first = tokens.next().expect("node name not found");
    mmap.write_nodekind(FdtNodeKind::BeginNode);
    if util::consume(tokens, "{") {
        let node_name = first;
        mmap.write_nodename(node_name);
        mmap.current_label = None;
    } else {
        let node_name = tokens.next().expect("node name not found");
        mmap.write_nodename(node_name);
        mmap.current_label = Some(first.trim_end_matches(':').to_string());
        util::expect(tokens, "{");
    }

    loop {
        parse_line(lines, mmap);

        if util::consume(lines, "};") {
            if mmap.strings.phandle_needed {
                mmap.write_property(
                    "phandle",
                    &mut vec![mmap.strings.phandle_value],
                    4,
                );
                mmap.strings.phandle_needed = false;
                mmap.strings.phandle_value += 1;
            }

            mmap.write_nodekind(FdtNodeKind::EndNode);
            break;
        }
    }
}

pub fn parse_line(lines: &mut Peekable<std::str::Lines>, mmap: &mut dtb_mmap) {
    if !util::consume(lines, "") {
        if lines.peek().unwrap().ends_with('{') {
            parse_node(lines, mmap);
        } else {
            parse_property(lines, mmap);
        }
    }
}

