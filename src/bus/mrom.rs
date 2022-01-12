use crate::elfload;
use super::Device;

pub struct Mrom {
        mrom: Vec<u8>,
    pub base_addr: u32,
}

