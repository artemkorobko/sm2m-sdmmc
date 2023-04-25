use core::ops::Deref;

use stm32f1xx_hal::{device, gpio};

use super::io;

pub struct SM2MGPIOCMap(device::GPIOC);

impl SM2MGPIOCMap {
    pub fn configure(
        pc3: gpio::PC3,
        pc6: gpio::PC6,
        pc7: gpio::PC7,
        pc8: gpio::PC8,
        pc9: gpio::PC9,
        pc10: gpio::PC10,
        pc11: gpio::PC11,
        pc12: gpio::PC12,
        crl: &mut gpio::Cr<'C', false>,
        crh: &mut gpio::Cr<'C', true>,
    ) -> SM2MGPIOCMap {
        pc3.into_push_pull_output(crl); // SETE
        pc6.into_push_pull_output(crl); // DO_9
        pc7.into_push_pull_output(crl); // DO_6
        pc8.into_push_pull_output(crh); // DO_5
        pc9.into_push_pull_output(crh); // DO_4
        pc10.into_push_pull_output(crh); // DO_0
        pc11.into_push_pull_output(crh); // DTEO
        pc12.into_push_pull_output(crh); // CTRLD
        SM2MGPIOCMap(unsafe { device::Peripherals::steal() }.GPIOC)
    }
}

impl io::SM2MBusRead for SM2MGPIOCMap {
    fn read(&self, data: io::InputData) -> io::InputData {
        data
    }
}

impl io::SM2MBusWrite for SM2MGPIOCMap {
    fn write(&mut self, data: &io::OutputData) {
        self.0.odr.modify(|r, w| {
            let mut bits = r.bits() & 0xFFFFE037; // read output register and clean bits 3, 6, 7, 8, 9, 10, 11, 12
            bits |= (data.deref() & 0x400000) >> 19; // set SETE (bit 22) to bit 3
            bits |= (data.deref() & 0x200) >> 3; // set DO_9 (bit 9) to bit 6
            bits |= (data.deref() & 0x40) << 1; // set DO_6 (bit 6) to bit 7
            bits |= (data.deref() & 0x20) << 3; // set DO_5 (bit 5) to bit 8
            bits |= (data.deref() & 0x10) << 5; // set DO_4 (bit 4) to bit 9
            bits |= (data.deref() & 0x1) << 10; // set DO_0 (bit 0) to bit 10
            bits |= (data.deref() & 0x800000) << 12; // set DTEO (bit 23) to bit 11
            bits |= (data.deref() & 0x80000) << 8; // set CTRLD (bit 19) to bit 11
            unsafe { w.bits(bits) }
        });
    }
}
