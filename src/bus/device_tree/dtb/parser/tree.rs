use super::super::LabelManager;
use super::{util, FdtTokenKind, Token};
use std::iter::Peekable;

fn parse_data(
    data: &str,
    label_mgr: &mut LabelManager,
) -> (Option<Vec<u32>>, Option<String>, usize) {
    if !data.ends_with(';') {
        panic!("{data} <-- ';' expected.");
    }

    let data_ch = &mut data.chars().peekable();
    match data_ch.next().unwrap() {
        '"' => {
            let mut str_data = String::new();
            loop {
                str_data = format!(
                    "{}{}\0",
                    str_data,
                    data_ch.take_while(|c| *c != '"').collect::<String>()
                );
                if util::consume(data_ch, ',') {
                    util::consume(data_ch, ' ');
                    util::expect(data_ch, '"');
                } else {
                    break;
                }
            }

            let size = str_data.len();
            let bin = str_data
                .into_bytes()
                .chunks(4)
                .map(|bs| {
                    // &[u8] -> [u8; 4]
                    let mut s = [0; 4];
                    s[..bs.len()].clone_from_slice(bs);
                    u32::from_be_bytes(s)
                })
                .collect::<Vec<u32>>();

            (Some(bin), None, size)
        }
        '<' => {
            let bin = data_ch
                .take_while(|c| *c != '>')
                .collect::<String>()
                .split(' ')
                .filter(|&x| !x.is_empty())
                .map(|num| {
                    if let Some(hex) = num.strip_prefix("0x") {
                        u32::from_str_radix(hex, 16).expect("parsing hex error.")
                    } else {
                        num.parse::<u32>().unwrap_or_else(|_| {
                            label_mgr.regist_phandle(num.trim_start_matches('&'))
                        })
                    }
                })
                .collect::<Vec<u32>>();
            let size = bin.len() * 4;

            (Some(bin), None, size)
        }
        '&' => {
            let label_name = data_ch.take_while(|c| *c != ';').collect::<String>();
            let size = label_name.len();
            (None, Some(label_name), size)
        }
        _ => panic!("prop data is invalid"),
    }
}

fn parse_property(lines: &mut Peekable<std::str::Lines>, label_mgr: &mut LabelManager) -> Token {
    let tokens = &mut util::tokenize(lines, "property is invalid").peekable();
    let name = tokens
        .next()
        .expect("prop name not found")
        .trim_start()
        .to_string();
    if util::consume(tokens, "=") {
        let raw_data = tokens.collect::<Vec<_>>().join(" ");
        let (data, label, size) = parse_data(&raw_data, label_mgr);
        Token {
            kind: FdtTokenKind::Prop,
            name,
            data,
            size: Some(size),
            label,
            child: None,
        }
    } else {
        let name = name.trim_end_matches(';').to_string();
        Token::no_data_prop(name)
    }
}

pub fn parse_node(
    lines: &mut Peekable<std::str::Lines>,
    label_mgr: &mut LabelManager,
    node_path: String,
) -> Token {
    let tokens = &mut util::tokenize(lines, "node is invalid").peekable();

    let first = tokens.next().expect("node name not found");
    let name = if util::consume(tokens, "{") {
        first.trim_start().to_string()
    } else {
        let node_name = tokens
            .next()
            .expect("node name not found")
            .trim_start()
            .to_string();
        let node_label = first.trim_end_matches(':');
        label_mgr.regist_label(node_label, format!("{node_path}/{node_name}"));

        util::expect(tokens, "{");
        node_name
    };

    let mut child: Vec<Token> = Vec::new();
    while !util::consume_ends_with(lines, "};") {
        // skip empty line
        if !util::consume(lines, "") {
            if name == "/" {
                child.push(parse_token(lines, label_mgr, node_path.clone()))
            } else {
                child.push(parse_token(lines, label_mgr, format!("{node_path}/{name}")))
            }
        }
    }

    Token {
        kind: FdtTokenKind::BeginNode,
        name,
        data: None,
        size: None,
        label: None,
        child: if child.is_empty() { None } else { Some(child) },
    }
}

fn parse_token(
    lines: &mut Peekable<std::str::Lines>,
    label_mgr: &mut LabelManager,
    node_path: String,
) -> Token {
    if lines.peek().unwrap().ends_with('{') {
        parse_node(lines, label_mgr, node_path)
    } else {
        parse_property(lines, label_mgr)
    }
}
