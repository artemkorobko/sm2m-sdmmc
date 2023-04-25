use stm32f1xx_hal::{afio, device, prelude::*};

use super::{io, prelude::*};

pub struct SM2MBus {
    gpioa_map: SM2MGPIOAMap,
    gpiob_map: SM2MGPIOBMap,
    gpioc_map: SM2MGPIOCMap,
    gpiod_map: SM2MGPIODMap,
    gpioe_map: SM2MGPIOEMap,
}

impl SM2MBus {
    pub fn configure(
        gpioa: device::GPIOA,
        gpiob: device::GPIOB,
        gpioc: device::GPIOC,
        gpiod: device::GPIOD,
        gpioe: device::GPIOE,
        afio: &mut afio::Parts,
        exti: &mut device::EXTI,
    ) -> Self {
        let mut gpioa = gpioa.split();
        let mut gpiob = gpiob.split();
        let mut gpioc = gpioc.split();
        let mut gpiod = gpiod.split();
        let mut gpioe = gpioe.split();

        let (pa15, _, pb4) = afio.mapr.disable_jtag(gpioa.pa15, gpiob.pb3, gpiob.pb4);

        Self {
            gpioa_map: SM2MGPIOAMap::configure(
                gpioa.pa8,
                gpioa.pa9,
                gpioa.pa10,
                gpioa.pa11,
                gpioa.pa12,
                pa15,
                &mut gpioa.crh,
            ),
            gpiob_map: SM2MGPIOBMap::configure(
                pb4,
                gpiob.pb6,
                gpiob.pb7,
                gpiob.pb8,
                gpiob.pb9,
                gpiob.pb12,
                gpiob.pb13,
                gpiob.pb14,
                gpiob.pb15,
                &mut gpiob.crl,
                &mut gpiob.crh,
                afio,
                exti,
            ),
            gpioc_map: SM2MGPIOCMap::configure(
                gpioc.pc3,
                gpioc.pc6,
                gpioc.pc7,
                gpioc.pc8,
                gpioc.pc9,
                gpioc.pc10,
                gpioc.pc11,
                gpioc.pc12,
                &mut gpioc.crl,
                &mut gpioc.crh,
            ),
            gpiod_map: SM2MGPIODMap::configure(
                gpiod.pd0,
                gpiod.pd1,
                gpiod.pd2,
                gpiod.pd3,
                gpiod.pd4,
                gpiod.pd5,
                gpiod.pd6,
                gpiod.pd7,
                gpiod.pd8,
                gpiod.pd9,
                gpiod.pd10,
                gpiod.pd11,
                gpiod.pd12,
                gpiod.pd13,
                gpiod.pd14,
                gpiod.pd15,
                &mut gpiod.crl,
                &mut gpiod.crh,
            ),
            gpioe_map: SM2MGPIOEMap::configure(
                gpioe.pe0,
                gpioe.pe1,
                gpioe.pe2,
                gpioe.pe3,
                gpioe.pe4,
                gpioe.pe5,
                gpioe.pe6,
                &mut gpioe.crl,
            ),
        }
    }
}

impl io::SM2MBusRead for SM2MBus {
    fn read(&self, mut data: io::InputData) -> super::io::InputData {
        data = self.gpioa_map.read(data);
        data = self.gpiob_map.read(data);
        data = self.gpioc_map.read(data);
        data = self.gpiod_map.read(data);
        data = self.gpioe_map.read(data);
        data
    }
}

impl io::SM2MBusWrite for SM2MBus {
    fn write(&mut self, data: &io::OutputData) {
        self.gpioa_map.write(data);
        self.gpiob_map.write(data);
        self.gpioc_map.write(data);
        self.gpiod_map.write(data);
        self.gpioe_map.write(data);
        self.gpiod_map.flush(data);
    }
}
