const TLB_ENTRIES: usize = 256;
const PGSHIFT: usize = 12;

#[derive(Clone)]
struct TlbEntry {
    _vaddr: u64,
    paddr: u64,
}

pub struct Tlb {
    tlb_tags: Vec<u64>,
    tlb_data: Vec<Option<TlbEntry>>,
}

impl Tlb {
    pub fn new() -> Self {
        Tlb {
            tlb_tags: vec![0; TLB_ENTRIES],
            tlb_data: vec![None; TLB_ENTRIES],
        }
    }

    pub fn lookup(&self, vaddr: u64) -> Option<u64> {
        let vpn = vaddr >> PGSHIFT;
        let index = (vpn % TLB_ENTRIES as u64) as usize;
        if self.tlb_tags[vpn as usize % TLB_ENTRIES] == vpn {
            self.tlb_data[index].as_ref().map(|entry| entry.paddr)
        } else {
            None
        }
    }

    pub fn flush(&mut self) {
        //self.tlb_tags.clear();
        //self.tlb_data.clear();
        //self.tlb_tags = vec![0; TLB_ENTRIES];
        //self.tlb_data = vec![None; TLB_ENTRIES];
        println!(
            "before: {}",
            self.tlb_data.iter().filter(|x| x.is_some()).count()
        );
        crate::log::infoln!("tlb flushed");

        for t in &mut self.tlb_tags {
            *t = 0;
        }
        for d in &mut self.tlb_data {
            *d = None;
        }

        println!(
            "after: {}",
            self.tlb_data.iter().filter(|x| x.is_some()).count()
        )
    }

    pub fn refill_tlb(&mut self, vaddr: u64, paddr: u64) {
        let index = ((vaddr >> PGSHIFT) % TLB_ENTRIES as u64) as usize;
        let expected_tag = vaddr >> PGSHIFT;
        let new_entry = TlbEntry {
            _vaddr: vaddr,
            paddr,
        };

        self.tlb_tags[index] = expected_tag;
        self.tlb_data[index] = Some(new_entry);
    }
}
