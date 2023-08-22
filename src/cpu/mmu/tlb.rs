use crate::cpu::TransFor;

const TLB_ENTRIES: usize = 256;
const PGSHIFT: usize = 12;

#[derive(Clone)]
struct TlbEntry {
    page_addr: u64,
}

pub struct Tlb {
    tlb_fetch_tags: Vec<Option<u64>>,
    tlb_load_tags: Vec<Option<u64>>,
    tlb_store_tags: Vec<Option<u64>>,
    tlb_data: Vec<Option<TlbEntry>>,
}

impl Tlb {
    pub fn new() -> Self {
        Tlb {
            tlb_fetch_tags: vec![None; TLB_ENTRIES],
            tlb_load_tags: vec![None; TLB_ENTRIES],
            tlb_store_tags: vec![None; TLB_ENTRIES],
            tlb_data: vec![None; TLB_ENTRIES],
        }
    }

    pub fn lookup(&self, vaddr: u64, purpose: TransFor) -> Option<u64> {
        let vpn = vaddr >> PGSHIFT;
        let index = (vpn % TLB_ENTRIES as u64) as usize;
        let tlb_tag = match purpose {
            TransFor::Fetch | TransFor::Deleg => self.tlb_fetch_tags[index],
            TransFor::Load => self.tlb_load_tags[index],
            TransFor::StoreAMO => self.tlb_store_tags[index],
        };
        if tlb_tag == Some(vpn) {
            self.tlb_data[index].as_ref().map(|entry| entry.page_addr)
        } else {
            None
        }
    }

    pub fn refill_tlb(&mut self, vaddr: u64, paddr: u64, purpose: TransFor) {
        let index = ((vaddr >> PGSHIFT) % TLB_ENTRIES as u64) as usize;
        let expected_tag = vaddr >> PGSHIFT;
        let new_entry = TlbEntry {
            page_addr: paddr >> PGSHIFT,
        };

        match purpose {
            TransFor::Fetch | TransFor::Deleg => self.tlb_fetch_tags[index] = Some(expected_tag),
            TransFor::Load => self.tlb_load_tags[index] = Some(expected_tag),
            TransFor::StoreAMO => self.tlb_store_tags[index] = Some(expected_tag),
        };
        self.tlb_data[index] = Some(new_entry);
    }

    pub fn flush(&mut self) {
        //self.tlb_tags.clear();
        //self.tlb_data.clear();
        //self.tlb_tags = vec![0; TLB_ENTRIES];
        //self.tlb_data = vec![None; TLB_ENTRIES];

        //println!(
        //    "before: {}",
        //    self.tlb_data.iter().filter(|x| x.is_some()).count()
        //);
        crate::log::infoln!("tlb flushed");

        for t in &mut self.tlb_fetch_tags {
            *t = None;
        }
        for t in &mut self.tlb_load_tags {
            *t = None;
        }
        for t in &mut self.tlb_store_tags {
            *t = None;
        }
        for d in &mut self.tlb_data {
            *d = None;
        }

        //println!(
        //    "after: {}",
        //    self.tlb_data.iter().filter(|x| x.is_some()).count()
        //)
    }
}
