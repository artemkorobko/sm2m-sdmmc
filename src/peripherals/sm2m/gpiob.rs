use stm32f1xx_hal::{
    afio, device,
    gpio::{self, ExtiPin},
};

pub struct SM2MGPIOBInterface(device::GPIOB);

impl SM2MGPIOBInterface {
    pub fn configure(
        pb4: gpio::PB4,
        pb6: gpio::PB6,
        pb7: gpio::PB7,
        pb8: gpio::PB8,
        pb9: gpio::PB9,
        pb13: gpio::PB13,
        pb14: gpio::PB14,
        crl: &mut gpio::Cr<'B', false>,
        crh: &mut gpio::Cr<'B', true>,
        afio: &mut afio::Parts,
        exti: &mut device::EXTI,
    ) -> SM2MGPIOBInterface {
        pb4.into_pull_down_input(crl); // DI_8
        pb6.into_pull_down_input(crl); // DTSI
        pb7.into_pull_down_input(crl); // DI_0
        pb8.into_pull_down_input(crh); // CTRLI_1
        pb9.into_pull_down_input(crh); // RST
        let mut trigger = pb13.into_pull_down_input(crh); // DTLI
        trigger.make_interrupt_source(afio);
        trigger.enable_interrupt(exti);
        trigger.trigger_on_edge(exti, gpio::Edge::Falling);
        pb14.into_pull_down_input(crh); // DTEI
        SM2MGPIOBInterface(unsafe { device::Peripherals::steal() }.GPIOB)
    }
}
