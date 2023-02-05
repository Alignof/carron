mod tree;
mod util;

use super::LabelManager;

#[allow(dead_code)]
pub enum FdtTokenKind {
    BeginNode = 0x1,
    EndNode = 0x2,
    Prop = 0x3,
    Nop = 0x4,
    End = 0x9,
}

pub struct Token {
    pub kind: FdtTokenKind,
    pub name: String,
    pub data: Option<Vec<u32>>,
    pub size: Option<usize>,
    pub label: Option<String>,
    pub child: Option<Vec<Token>>,
}

impl Token {
    pub fn no_data_prop(name: String) -> Self {
        Token {
            kind: FdtTokenKind::Prop,
            name,
            data: Some(vec![]),
            size: Some(0),
            label: None,
            child: None,
        }
    }
}

pub fn make_tree(dts: String, label_mgr: &mut LabelManager) -> Token {
    let mut lines = dts.lines().peekable();

    if lines.next() != Some("/dts-v1/;") {
        panic!("version isn't specified");
    }
    util::consume(&mut lines, "");

    tree::parse_node(&mut lines, label_mgr, String::new())
}
