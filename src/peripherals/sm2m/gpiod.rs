use stm32f1xx_hal::{device, gpio};

pub struct SM2MGPIODInterface(gpio::Pin<'D', 10, gpio::Output>, device::GPIOD);

impl SM2MGPIODInterface {
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
    ) -> SM2MGPIODInterface {
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
        let pd10 = pd10.into_push_pull_output(crh); // RDY
        pd11.into_push_pull_output(crh); // DO_12
        pd12.into_push_pull_output(crh); // DO_13
        pd13.into_push_pull_output(crh); // DO_10
        pd14.into_push_pull_output(crh); // DO_8
        pd15.into_push_pull_output(crh); // DO_7
        SM2MGPIODInterface(pd10, unsafe { device::Peripherals::steal() }.GPIOD)
    }
}
