use stm32f1xx_hal::gpio;

use super::*;

pub struct SM2MBus {
    gpioa_map: SM2MGPIOAMap,
    gpiob_map: SM2MGPIOBMap,
    gpioc_map: SM2MGPIOCMap,
    gpiod_map: SM2MGPIODMap,
    gpioe_map: SM2MGPIOEMap,
}

impl SM2MBus {
    pub fn configure(
        gpioa_config: GPIOAConfig,
        gpiob_config: GPIOBConfig,
        gpioc_config: GPIOCConfig,
        gpiod_config: GPIODConfig,
        gpioe_config: GPIOEConfig,
    ) -> Self {
        Self {
            gpioa_map: SM2MGPIOAMap::configure(gpioa_config),
            gpiob_map: SM2MGPIOBMap::configure(gpiob_config),
            gpioc_map: SM2MGPIOCMap::configure(gpioc_config),
            gpiod_map: SM2MGPIODMap::configure(gpiod_config),
            gpioe_map: SM2MGPIOEMap::configure(gpioe_config),
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
