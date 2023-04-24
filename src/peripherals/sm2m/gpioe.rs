use stm32f1xx_hal::{device, gpio};

pub struct SM2MGPIOEInterface(device::GPIOE);

impl SM2MGPIOEInterface {
    pub fn configure(
        pe0: gpio::PE0,
        pe1: gpio::PE1,
        pe2: gpio::PE2,
        pe3: gpio::PE3,
        pe4: gpio::PE4,
        pe5: gpio::PE5,
        pe6: gpio::PE6,
        crl: &mut gpio::Cr<'E', false>,
    ) -> SM2MGPIOEInterface {
        pe0.into_pull_down_input(crl); // DI_2
        pe1.into_pull_down_input(crl); // DI_1
        pe2.into_pull_down_input(crl); // DI_3
        pe3.into_pull_down_input(crl); // DI_4
        pe4.into_pull_down_input(crl); // DI_6
        pe5.into_pull_down_input(crl); // DI_5
        pe6.into_pull_down_input(crl); // DI_7
        SM2MGPIOEInterface(unsafe { device::Peripherals::steal() }.GPIOE)
    }
}
