use core::ops::{Deref, DerefMut};

use stm32f1xx_hal::{device, gpio};

use super::io;

pub struct SM2MGPIODMap(device::GPIOD);

impl SM2MGPIODMap {
    pub fn configure(
        pd0: gpio::PD0,
        pd1: gpio::PD1,
        pd2: gpio::PD2,
        pd3: gpio::PD3,
        pd4: gpio::PD4,
        pd5: gpio::PD5,
        pd6: gpio::PD6,
        pd7: gpio::PD7,
        pd8: gpio::PD8,
        pd9: gpio::PD9,
        pd10: gpio::PD10,
        pd11: gpio::PD11,
        pd12: gpio::PD12,
        pd13: gpio::PD13,
        pd14: gpio::PD14,
        pd15: gpio::PD15,
        crl: &mut gpio::Cr<'D', false>,
        crh: &mut gpio::Cr<'D', true>,
    ) -> SM2MGPIODMap {
        pd0.into_pull_down_input(crl); // DI_15
        pd1.into_pull_down_input(crl); // DI_13
        pd2.into_pull_down_input(crl); // CTRLI_0
        pd3.into_pull_down_input(crl); // DI_14
        pd4.into_pull_down_input(crl); // DI_12
        pd5.into_pull_down_input(crl); // DI_10
        pd6.into_pull_down_input(crl); // DI_9
        pd7.into_pull_down_input(crl); // DI_11
        pd8.into_push_pull_output(crh); // DO_11
        pd9.into_push_pull_output(crh); // DO_15
        pd10.into_push_pull_output(crh); // RDY
        pd11.into_push_pull_output(crh); // DO_12
        pd12.into_push_pull_output(crh); // DO_13
        pd13.into_push_pull_output(crh); // DO_10
        pd14.into_push_pull_output(crh); // DO_8
        pd15.into_push_pull_output(crh); // DO_7
        SM2MGPIODMap(unsafe { device::Peripherals::steal() }.GPIOD)
    }

    pub fn flush(&mut self, data: &io::OutputData) {
        self.0.odr.modify(|r, w| {
            let mut bits = r.bits() & 0xFFFFFBFF; // read output register and clean bit 10
            bits |= (data.deref() & 0x40000) >> 8; // set RDY (bit 18) to bit 10
            unsafe { w.bits(bits) }
        });
    }
}

impl io::SM2MBusRead for SM2MGPIODMap {
    fn read(&self, mut data: io::InputData) -> io::InputData {
        let bits = self.0.idr.read().bits();
        *data.deref_mut() |= (bits & 0x1) << 15; // set bit 0 to DI_15 (bit 15)
        *data.deref_mut() |= (bits & 0x2) << 12; // set bit 1 to DI_13 (bit 13)
        *data.deref_mut() |= (bits & 0x4) << 12; // set bit 2 to CTRLI_0 (bit 16)
        *data.deref_mut() |= (bits & 0x8) << 11; // set bit 3 to DI_14 (bit 14)
        *data.deref_mut() |= (bits & 0x10) << 8; // set bit 4 to DI_12 (bit 12)
        *data.deref_mut() |= (bits & 0x20) << 5; // set bit 5 to DI_10 (bit 10)
        *data.deref_mut() |= (bits & 0x40) << 3; // set bit 6 to DI_9 (bit 9)
        *data.deref_mut() |= (bits & 0x80) << 2; // set bit 7 to DI_11 (bit 9)
        data
    }
}

impl io::SM2MBusWrite for SM2MGPIODMap {
    fn write(&mut self, data: &io::OutputData) {
        self.0.odr.modify(|r, w| {
            let mut bits = r.bits() & 0xFFFF04FF; // read output register and clean bits 8, 9, 11, 12, 13, 14, 15
            bits |= (data.deref() & 0x800) >> 3; // set DO_11 (bit 11) to bit 8
            bits |= (data.deref() & 0x8000) >> 6; // set DO_15 (bit 15) to bit 9
            bits |= (data.deref() & 0x1000) >> 1; // set DO_12 (bit 12) to bit 11
            bits |= (data.deref() & 0x2000) >> 1; // set DO_13 (bit 13) to bit 12
            bits |= (data.deref() & 0x400) << 3; // set DO_10 (bit 10) to bit 13
            bits |= (data.deref() & 0x100) << 6; // set DO_8 (bit 8) to bit 14
            bits |= (data.deref() & 0x80) << 7; // set DO_7 (bit 7) to bit 14
            unsafe { w.bits(bits) }
        });
    }
}
