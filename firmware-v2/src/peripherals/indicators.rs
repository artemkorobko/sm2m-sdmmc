use stm32f1xx_hal::gpio;

pub type Pin<const P: char, const N: u8> = gpio::Pin<P, N, gpio::Output<gpio::PushPull>>;

pub struct Pins {
    pub system_error: Pin<'A', 0>,
    pub write: Pin<'A', 1>,
    pub read: Pin<'A', 2>,
}

pub struct Indicators {
    pins: Pins,
}

impl Indicators {
    pub fn new(pins: Pins) -> Self {
        Self { pins }
    }

    pub fn system_error_on(&mut self) {
        self.pins.system_error.set_low();
    }

    pub fn system_error_off(&mut self) {
        self.pins.system_error.set_high();
    }

    pub fn write_on(&mut self) {
        self.pins.write.set_low();
    }

    pub fn write_off(&mut self) {
        self.pins.write.set_high();
    }

    pub fn read_on(&mut self) {
        self.pins.read.set_low();
    }

    pub fn read_off(&mut self) {
        self.pins.read.set_high();
    }
}
