mod io;

use super::Device;
use crate::TrapCause;
use std::collections::VecDeque;

const UART_QUEUE_SIZE: usize = 64;
const MAX_BACKOFF: u64 = 16;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum UartRegister {
    RX_TX,   // read: RX, write: TX
    IER,     // write: IER
    IIR_FCR, // read: IIR, write: FCR
    LCR,     // write: LCR
    MCR,     // write: MCR
    LSR,     // read: LSR
    MSR,     // read: MSR
    SCR,     // I/O: SCR
}

#[allow(dead_code)]
enum IerMask {
    RDI = 0x1,
    THRI = 0x2,
    RLSI = 0x4,
    MSI = 0x8,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum IirFcrMask {
    MSI = 0x00,
    NO_INT = 0x01,
    THRI = 0x02,
    RDI = 0x04,
    RLSI = 0x06,
    ID = 0x0e,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum LcrMask {
    DLAB = 0x80,
    SBC = 0x40,
    SPAR = 0x20,
    EPAR = 0x10,
    PARITY = 0x08,
    STOP = 0x04,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum FcrMask {
    ENABLE_FIFO = 0x01,
    CLEAR_RCVR = 0x02,
    CLEAR_XMIT = 0x04,
    DMA_SELECT = 0x08,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum McrMask {
    LOOP = 0x10,
    OUT2 = 0x08,
    OUT1 = 0x04,
    RTS = 0x02,
    DTR = 0x01,
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum LsrMask {
    FIFOE = 0x80,
    TEMT = 0x40,
    THRE = 0x20,
    BI = 0x10,
    FE = 0x08,
    PE = 0x04,
    OE = 0x02,
    DR = 0x01,
    BRK_ERROR_BITS = 0x1E,
}

pub struct Uart {
    pub uart: Vec<u8>,
    rx_queue: VecDeque<u8>,
    backoff_counter: u64,
    pub base_addr: u64,
    size: usize,
}

impl Default for Uart {
    fn default() -> Self {
        Self::new()
    }
}

impl Uart {
    pub fn new() -> Self {
        const UART_SIZE: usize = 0x100;
        let mut uart = vec![0; UART_SIZE];
        uart[UartRegister::IIR_FCR as usize] = 0x1; // IIR_NO_INT
        uart[UartRegister::LSR as usize] = 0x60; // LSR_TEMT | LSR_THRE
        uart[UartRegister::MSR as usize] = 0xb0; // UART_MSR_DCD | UART_MSR_DSR | UART_MSR_CTS
        uart[UartRegister::MCR as usize] = 0x08; // MCR_OUT2

        Uart {
            uart,
            rx_queue: VecDeque::new(),
            backoff_counter: 0,
            base_addr: 0x1000_0000,
            size: UART_SIZE,
        }
    }

    pub fn tick(&mut self) {
        if (self.uart[UartRegister::IIR_FCR as usize] & FcrMask::ENABLE_FIFO as u8 == 0)
            || (self.uart[UartRegister::MCR as usize] & McrMask::LOOP as u8 != 0)
            || (UART_QUEUE_SIZE <= self.rx_queue.len())
        {
            return;
        }

        if self.backoff_counter > 0 && self.backoff_counter < MAX_BACKOFF {
            self.backoff_counter += 1;
            return;
        }

        let input = '\0'; // input value here
        if (input as i8) < 0 {
            self.backoff_counter = 1;
            return;
        }

        self.backoff_counter = 0;

        self.rx_queue.push_back(input as u8);
        self.uart[UartRegister::LSR as usize] |= LsrMask::DR as u8;
    }
}

#[allow(clippy::identity_op)]
impl Device for Uart {
    // is addr in device address space
    fn in_range(&self, addr: u64) -> bool {
        (self.base_addr..=self.base_addr + self.size as u64).contains(&addr)
    }

    // address to raw index
    fn addr2index(&self, addr: u64) -> usize {
        (addr - self.base_addr) as usize
    }

    // store
    fn store8(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        const TX: usize = UartRegister::RX_TX as usize;
        const IER: usize = UartRegister::IER as usize;
        const FCR: usize = UartRegister::IIR_FCR as usize;
        const LCR: usize = UartRegister::LCR as usize;
        const MCR: usize = UartRegister::MCR as usize;
        let index = self.addr2index(addr);

        match index {
            TX => self.tx_byte(char::from_u32(data as u32).unwrap()),
            _ => self.uart[index] = (data & 0xFF) as u8,
        }

        match index {
            IER | FCR | LCR | MCR => self.update_interrupt(),
            _ => (),
        }

        Ok(())
    }

    fn store16(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            "uart only allows load/store8 but try store16".to_string(),
        ))
    }

    fn store32(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            "uart only allows load/store8 but try store32".to_string(),
        ))
    }

    fn store64(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            "uart only allows load/store8 but try store64".to_string(),
        ))
    }

    // load
    fn load8(&mut self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        const RX: usize = UartRegister::RX_TX as usize;
        let index = self.addr2index(addr);

        match index {
            RX => {
                let data = if self.uart[UartRegister::LCR as usize] & LcrMask::DLAB as u8 != 0 {
                    self.uart[UartRegister::LCR as usize] & LcrMask::DLAB as u8
                } else {
                    self.rx_byte()
                };
                self.update_interrupt();

                Ok(data as i8 as i64 as u64)
            }
            _ => Ok(self.uart[index] as i8 as i64 as u64),
        }
    }

    fn load16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "uart only allows load/store8 but try load16".to_string(),
        ))
    }

    fn load32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "uart only allows load/store8 but try load32".to_string(),
        ))
    }

    fn load64(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "uart only allows load/store8 but try load64".to_string(),
        ))
    }

    fn load_u8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let index = self.addr2index(addr);
        Ok(self.uart[index] as u64)
    }

    fn load_u16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "uart only allows load/store8 but try load_u16".to_string(),
        ))
    }

    fn load_u32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "uart only allows load/store8 but try load_u32".to_string(),
        ))
    }
}
