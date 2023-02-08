use super::{Uart, UartLsr, UartRegister};

impl Uart {
    pub fn tx_byte(&mut self, data: char) {
        self.uart[UartRegister::LSR as usize] |=
            (1 << UartLsr::TEMT as u8) | (1 << UartLsr::THRE as u8);
        print!("{}", char::from_u32(data as u32).unwrap());
    }
}
