use super::{FcrMask, IerMask, IirFcrMask, LsrMask, Uart, UartRegister};

impl Uart {
    pub fn update_interrupt(&mut self) {
        let mut interrupts = 0;
        if self.uart[UartRegister::LCR as usize] & FcrMask::CLEAR_RCVR as u8 != 0 {
            self.uart[UartRegister::LCR as usize] &= !(FcrMask::CLEAR_RCVR as u8);
            self.rx_queue.clear();
            self.uart[UartRegister::LSR as usize] &= !(LsrMask::DR as u8);
        }

        if self.uart[UartRegister::LCR as usize] & FcrMask::CLEAR_XMIT as u8 != 0 {
            self.uart[UartRegister::LCR as usize] &= !(FcrMask::CLEAR_XMIT as u8);
            self.uart[UartRegister::LSR as usize] |= LsrMask::TEMT as u8 | LsrMask::THRE as u8;
        }

        if self.uart[UartRegister::IER as usize] & IerMask::RDI as u8 != 0
            && self.uart[UartRegister::LSR as usize] & LsrMask::DR as u8 != 0
        {
            interrupts |= IirFcrMask::RDI as u8;
        }

        if self.uart[UartRegister::IER as usize] & IerMask::THRI as u8 != 0
            && self.uart[UartRegister::LSR as usize] & LsrMask::TEMT as u8 != 0
        {
            interrupts |= IirFcrMask::THRI as u8;
        }

        if interrupts == 0 {
            self.uart[UartRegister::IIR_FCR as usize] = IirFcrMask::NO_INT as u8;
            self.interrupt_level = 0;
            //plic.set_interrupt_level(UART_INTERRUPT_ID, 0);
        } else {
            self.uart[UartRegister::IIR_FCR as usize] = interrupts;
            self.interrupt_level = 1;
            //plic.set_interrupt_level(UART_INTERRUPT_ID, 1);
        }

        if self.uart[UartRegister::IER as usize] & IerMask::THRI as u8 == 0 {
            self.uart[UartRegister::LSR as usize] |= LsrMask::TEMT as u8 | LsrMask::THRE as u8;
        }
    }

    pub fn rx_byte(&mut self) -> u8 {
        if self.rx_queue.is_empty() {
            self.uart[UartRegister::LSR as usize] &= !(LsrMask::DR as u8);
            return 0;
        }

        if self.uart[UartRegister::LSR as usize] & LsrMask::BI as u8 != 0 {
            self.uart[UartRegister::LSR as usize] &= !(LsrMask::BI as u8);
            return 0;
        }

        let front = self.rx_queue.pop_front().unwrap();
        if self.rx_queue.is_empty() {
            self.uart[UartRegister::LSR as usize] &= !(LsrMask::DR as u8);
        }

        front
    }

    pub fn tx_byte(&mut self, data: char) {
        self.uart[UartRegister::LSR as usize] |= (LsrMask::TEMT as u8) | (LsrMask::THRE as u8);
        print!("{}", char::from_u32(data as u32).unwrap());
    }
}
