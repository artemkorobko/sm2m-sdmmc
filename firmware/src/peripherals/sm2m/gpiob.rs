use core::ops::{Deref, DerefMut};

use stm32f1xx_hal::{
    afio, device,
    gpio::{self, ExtiPin},
};

use super::io;

pub struct SM2MGPIOBMap(device::GPIOB);

impl SM2MGPIOBMap {
    pub fn configure(
        pb4: gpio::PB4,
        pb6: gpio::PB6,
        pb7: gpio::PB7,
        pb8: gpio::PB8,
        pb9: gpio::PB9,
        pb12: gpio::PB12,
        pb13: gpio::PB13,
        pb14: gpio::PB14,
        pb15: gpio::PB15,
        crl: &mut gpio::Cr<'B', false>,
        crh: &mut gpio::Cr<'B', true>,
        afio: &mut afio::Parts,
        exti: &mut device::EXTI,
    ) -> SM2MGPIOBMap {
        pb4.into_pull_down_input(crl); // DI_8
        pb6.into_pull_down_input(crl); // DTSI
        pb7.into_pull_down_input(crl); // DI_0
        pb8.into_pull_down_input(crh); // CTRLI_1
        pb9.into_pull_down_input(crh); // RSTI
        pb12.into_push_pull_output(crh); // RSTE
        let mut trigger = pb13.into_pull_down_input(crh); // DTLI
        trigger.make_interrupt_source(afio);
        trigger.enable_interrupt(exti);
        trigger.trigger_on_edge(exti, gpio::Edge::Falling);
        pb14.into_pull_down_input(crh); // DTEI
        pb15.into_push_pull_output(crh); // DO_14
        SM2MGPIOBMap(unsafe { device::Peripherals::steal() }.GPIOB)
    }
}

impl io::SM2MBusRead for SM2MGPIOBMap {
    fn read(&self, mut data: io::InputData) -> io::InputData {
        let bits = self.0.idr.read().bits();
        *data.deref_mut() |= (bits & 0x10) << 4; // set bit 4 to DI_8 (bit 8)
                                                 // *data.deref_mut() |= (bits & 0x40) << 13; // set bit 6 to DTSI (bit 19) // used as interrupt
                                                 // *data.deref_mut() |= (bits & 0x2000) << 7; // set bit 13 to DTLI (bit 20) // not used in favor of DTSI
        *data.deref_mut() |= (bits & 0x80) >> 7; // set bit 7 to DI_0 (bit 0)
        *data.deref_mut() |= (bits & 0x100) << 9; // set bit 8 to CTRLI_1 (bit 17)
        *data.deref_mut() |= (bits & 0x200) << 9; // set bit 9 to RSTI (bit 18)
        *data.deref_mut() |= (bits & 0x4000) << 7; // set bit 14 to DTEI (bit 21)
        data
    }
}

impl io::SM2MBusWrite for SM2MGPIOBMap {
    fn write(&mut self, data: &io::OutputData) {
        self.0.odr.modify(|r, w| {
            let mut bits = r.bits() & 0xFFFF6FFF; // read output register and clean bits 12, 15
            bits |= (data.deref() & 0x200000) >> 9; // set RSTE (bit 21) to bit 12
            bits |= (data.deref() & 0x4000) << 1; // set DO_14 (bit 14) to bit 15
            unsafe { w.bits(bits) }
        });
    }
}
