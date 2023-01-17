mod generate;
mod label;
mod parser;

use generate::{create_mmap, write_dtb, DtbMmap};
use label::LabelManager;
use parser::FdtTokenKind;

pub fn make_dtb(dts: String) -> Vec<u8> {
    let mut label_mgr: LabelManager = LabelManager::new();
    let tree = parser::make_tree(dts, &mut label_mgr);
    let mmap = DtbMmap::new(label_mgr);

    let mut mmap = create_mmap::create_mmap(&tree, mmap);
    mmap.write_nodekind(FdtTokenKind::End);

    write_dtb::write_dtb(mmap)
}
