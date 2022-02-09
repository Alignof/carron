use std::iter::Peekable;
use super::util;
use super::dtb_mmap;

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
            let int_datas: Vec<u32> = data_ch
                .take_while(|c| *c != '>')
                .collect::<String>()
                .split(' ')
                .map(|num| {
                    if let Some(hex) = num.strip_prefix("0x") {
                        u32::from_str_radix(hex, 16).expect("parsing hex error.")
                    } else {
                        num.parse::<u32>().unwrap_or_else(|_| {
                            mmap.labels
                                .get(num.trim_start_matches('&'))
                                .expect("parsing integer error.")
                                .clone()
                        })
                    }
                })
                .collect::<Vec<u32>>();
        },
        _ => panic!("prop data is invalid"),
    }

    if data_ch.last() != Some(';') {
        panic!("{} <-- ';' expected.", data);
    }
}

pub fn parse_property(lines: &mut Peekable<std::str::Lines>, mmap: &mut dtb_mmap) {
    let tokens = &mut util::tokenize(lines, "property is invalid").peekable();
    let prop_name = tokens.next().expect("prop name not found");

    if util::consume(tokens, "=") {
        let raw_data = tokens.collect::<Vec<_>>().join(" ");
        parse_data(&raw_data, mmap);
    }
}

pub fn parse_node(lines: &mut Peekable<std::str::Lines>, mmap: &mut dtb_mmap) {
    dbg!(&lines.peek());
    // node
    if lines.peek().unwrap().chars().last() == Some('{') {
        // expect node's name and "{"
        let tokens = &mut util::tokenize(lines, "node is invalid").peekable();

        let first = tokens.next().expect("node name not found");
        if util::consume(tokens, "{") {
            mmap.current_label = None;
            let node_name = first; 
        } else {
            mmap.current_label = Some(first.to_string());
            let node_name = tokens.next().expect("node name not found");
            util::expect(tokens.next(), "{");
        }

        loop {
            parse_node(lines, mmap);

            if util::consume(lines, "};") {
                break;
            }
        }
    // property
    } else {
        parse_property(lines, mmap);
    }
}


