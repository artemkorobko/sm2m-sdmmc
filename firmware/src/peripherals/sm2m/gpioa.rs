use stm32f1xx_hal::{device, gpio};

pub struct SM2MGPIOAInterface(device::GPIOA);

impl SM2MGPIOAInterface {
    pub fn configure(
        pa8: gpio::PA8,
        pa9: gpio::PA9,
        pa10: gpio::PA10,
        pa11: gpio::PA11,
        pa12: gpio::PA12,
        pa15: gpio::PA15,
        crh: &mut gpio::Cr<'A', true>,
    ) -> SM2MGPIOAInterface {
        pa8.into_push_pull_output(crh); // DO_3
        pa9.into_push_pull_output(crh); // CTRLO_1
        pa10.into_push_pull_output(crh); // DO_2
        pa11.into_push_pull_output(crh); // CTRLO_0
        pa12.into_push_pull_output(crh); // DO_1
        pa15.into_push_pull_output(crh); // ERR
        SM2MGPIOAInterface(unsafe { device::Peripherals::steal() }.GPIOA)
    }

    pub fn write(&mut self, data: u16) {
        self.0.odr.modify(|r, w| {
            let mut bits = r.bits() & 0xFFFF60FF; // read all bits and clean ony required
            bits |= ((data & 0x2) << 11) as u32; // set DO_1 to bit 12
            bits |= ((data & 0x4) << 8) as u32; // set DO_2 to bit 10
            bits |= ((data & 0x8) << 5) as u32; // set DO_3 to bit 8
            // bits = write_flag(bits, 9, output.ctrlo_1); // pa9 - CTRLO_1
            // bits = write_flag(bits, 11, output.ctrlo_0); // pa11 - CTRLO_0
            // bits = write_flag(bits, 15, output.err); // pa15 - ERR
            unsafe { w.bits(bits) }
        });
    }
}
