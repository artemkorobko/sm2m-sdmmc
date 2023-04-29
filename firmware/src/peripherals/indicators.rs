use stm32f1xx_hal::gpio;

pub struct IndicatorPins<'a> {
    pub pa0: gpio::PA0,
    pub pa1: gpio::PA1,
    pub pa2: gpio::PA2,
    pub crl: &'a mut gpio::Cr<'A', false>,
}

pub struct Indicators {
    system_error: gpio::Pin<'A', 0, gpio::Output>,
    write: gpio::Pin<'A', 1, gpio::Output>,
    read: gpio::Pin<'A', 2, gpio::Output>,
}

impl Indicators {
    pub fn configure(config: IndicatorPins) -> Self {
        Self {
            system_error: config
                .pa0
                .into_push_pull_output_with_state(config.crl, gpio::PinState::High),
            write: config
                .pa1
                .into_push_pull_output_with_state(config.crl, gpio::PinState::High),
            read: config
                .pa2
                .into_push_pull_output_with_state(config.crl, gpio::PinState::High),
        }
    }

    pub fn system_error_on(&mut self) {
        self.system_error.set_low();
    }

    pub fn system_error_off(&mut self) {
        self.system_error.set_high();
    }

    pub fn write_on(&mut self) {
        self.write.set_low();
    }

    pub fn write_off(&mut self) {
        self.write.set_high();
    }

    pub fn read_on(&mut self) {
        self.read.set_low();
    }

    pub fn read_off(&mut self) {
        self.read.set_high();
    }

    pub fn all_on(&mut self) {
        self.system_error_on();
        self.write_on();
        self.read_on();
    }

    pub fn all_off(&mut self) {
        self.system_error_off();
        self.write_off();
        self.read_off();
    }
}
