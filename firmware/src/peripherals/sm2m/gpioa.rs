use core::ops::Deref;

use stm32f1xx_hal::{device, gpio};

use super::io;

pub struct SM2MGPIOAMap(device::GPIOA);

impl SM2MGPIOAMap {
    pub fn configure(
        pa8: gpio::PA8,
        pa9: gpio::PA9,
        pa10: gpio::PA10,
        pa11: gpio::PA11,
        pa12: gpio::PA12,
        pa15: gpio::PA15,
        crh: &mut gpio::Cr<'A', true>,
    ) -> SM2MGPIOAMap {
        pa8.into_push_pull_output(crh); // DO_3
        pa9.into_push_pull_output(crh); // CTRLO_1
        pa10.into_push_pull_output(crh); // DO_2
        pa11.into_push_pull_output(crh); // CTRLO_0
        pa12.into_push_pull_output(crh); // DO_1
        pa15.into_push_pull_output(crh); // ERRO
        SM2MGPIOAMap(unsafe { device::Peripherals::steal() }.GPIOA)
    }
}

impl io::SM2MBusRead for SM2MGPIOAMap {
    fn read(&self, data: io::InputData) -> io::InputData {
        data
    }
}

impl io::SM2MBusWrite for SM2MGPIOAMap {
    fn write(&mut self, data: &io::OutputData) {
        self.0.odr.modify(|r, w| {
            let mut bits = r.bits() & 0xFFFF60FF; // read output register and clean bits 8, 9, 10, 11, 12, 15
            bits |= (data.deref() & 0x8) << 5; // set DO_3 (bit 3) to bit 8
            bits |= (data.deref() & 0x20000) >> 8; // set CTRLO_1 (bit 17) to bit 9
            bits |= (data.deref() & 0x2) << 11; // set DO_1 (bit 1) to bit 12
            bits |= (data.deref() & 0x4) << 8; // set DO_2 (bit 2) to bit 10
            bits |= (data.deref() & 0x10000) >> 5; // set CTRLO_0 (bit 16) to bit 11
            bits |= (data.deref() & 0x100000) >> 5; // set ERRO (bit 20) to bit 15
            unsafe { w.bits(bits) }
        });
    }
}
