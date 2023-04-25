use core::ops::DerefMut;

use stm32f1xx_hal::{device, gpio};

use super::io;

pub struct SM2MGPIOEMap(device::GPIOE);

impl SM2MGPIOEMap {
    pub fn configure(
        pe0: gpio::PE0,
        pe1: gpio::PE1,
        pe2: gpio::PE2,
        pe3: gpio::PE3,
        pe4: gpio::PE4,
        pe5: gpio::PE5,
        pe6: gpio::PE6,
        crl: &mut gpio::Cr<'E', false>,
    ) -> SM2MGPIOEMap {
        pe0.into_pull_down_input(crl); // DI_2
        pe1.into_pull_down_input(crl); // DI_1
        pe2.into_pull_down_input(crl); // DI_3
        pe3.into_pull_down_input(crl); // DI_4
        pe4.into_pull_down_input(crl); // DI_6
        pe5.into_pull_down_input(crl); // DI_5
        pe6.into_pull_down_input(crl); // DI_7
        SM2MGPIOEMap(unsafe { device::Peripherals::steal() }.GPIOE)
    }
}

impl io::SM2MBusRead for SM2MGPIOEMap {
    fn read(&self, mut data: io::InputData) -> io::InputData {
        let bits = self.0.idr.read().bits();
        *data.deref_mut() |= (bits & 0x1) << 2; // set bit 0 to DI_2 (bit 2)
        *data.deref_mut() |= bits & 0x2; // set bit 1 to DI_1 (bit 1)
        *data.deref_mut() |= (bits & 0x4) << 1; // set bit 2 to DI_3 (bit 3)
        *data.deref_mut() |= (bits & 0x8) << 1; // set bit 3 to DI_4 (bit 4)
        *data.deref_mut() |= (bits & 0x10) << 2; // set bit 4 to DI_6 (bit 6)
        *data.deref_mut() |= bits & 0x20; // set bit 5 to DI_5 (bit 5)
        *data.deref_mut() |= (bits & 0x40) << 1; // set bit 6 to DI_7 (bit 7)
        data
    }
}

impl io::SM2MBusWrite for SM2MGPIOEMap {
    fn write(&mut self, _: &io::OutputData) {}
}
