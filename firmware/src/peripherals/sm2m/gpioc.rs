use stm32f1xx_hal::{device, gpio};

pub struct SM2MGPIOCInterface(device::GPIOC);

impl SM2MGPIOCInterface {
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
    ) -> SM2MGPIOCInterface {
        pc3.into_push_pull_output(crl); // SETE
        pc6.into_push_pull_output(crl); // DO_9
        pc7.into_push_pull_output(crl); // DO_6
        pc8.into_push_pull_output(crh); // DO_5
        pc9.into_push_pull_output(crh); // DO_4
        pc10.into_push_pull_output(crh); // DO_0
        pc11.into_push_pull_output(crh); // DTEO
        pc12.into_pull_down_input(crh); // CTRLD
        SM2MGPIOCInterface(unsafe { device::Peripherals::steal() }.GPIOC)
    }
}
