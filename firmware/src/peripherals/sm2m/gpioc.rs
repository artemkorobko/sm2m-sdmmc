use core::ops::Deref;

use stm32f1xx_hal::{device, gpio};

use super::io;

pub struct GPIOCConfig<'a> {
    pub pc3: gpio::PC3,
    pub pc6: gpio::PC6,
    pub pc7: gpio::PC7,
    pub pc8: gpio::PC8,
    pub pc9: gpio::PC9,
    pub pc10: gpio::PC10,
    pub pc11: gpio::PC11,
    pub pc12: gpio::PC12,
    pub crl: &'a mut gpio::Cr<'C', false>,
    pub crh: &'a mut gpio::Cr<'C', true>,
}

pub struct SM2MGPIOCMap(device::GPIOC);

impl SM2MGPIOCMap {
    pub fn configure(config: GPIOCConfig) -> Self {
        config.pc3.into_push_pull_output(config.crl); // SETE
        config.pc6.into_push_pull_output(config.crl); // DO_9
        config.pc7.into_push_pull_output(config.crl); // DO_6
        config.pc8.into_push_pull_output(config.crh); // DO_5
        config.pc9.into_push_pull_output(config.crh); // DO_4
        config.pc10.into_push_pull_output(config.crh); // DO_0
        config.pc11.into_push_pull_output(config.crh); // DTEO
        config.pc12.into_push_pull_output(config.crh); // CTRLD
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
