use super::{Uart, UartLsr, UartRegister};

impl Uart {
    pub fn rx_byte(&mut self) -> u8 {
        if self.rx_queue.is_empty() {
            self.uart[UartRegister::LSR as usize] &= !UartLsr::DR.mask();
            return 0;
        }

        if self.uart[UartRegister::LSR as usize] & UartLsr::BI.mask() != 0 {
            self.uart[UartRegister::LSR as usize] &= !(UartLsr::BI.mask());
            return 0;
        }

        let front = self.rx_queue.pop_front().unwrap();
        if self.rx_queue.is_empty() {
            self.uart[UartRegister::LSR as usize] &= !(UartLsr::DR.mask());
        }

        front
    }

    pub fn tx_byte(&mut self, data: char) {
        self.uart[UartRegister::LSR as usize] |= (UartLsr::TEMT.mask()) | (UartLsr::THRE.mask());
        print!("{}", char::from_u32(data as u32).unwrap());
    }
}
