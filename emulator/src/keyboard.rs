use stm32f1xx_hal::gpio;

pub struct Keys {
    pub reset: bool,
    pub run: bool,
    pub step: bool,
}

impl Keys {
    pub fn is_pressed(&self) -> bool {
        self.reset | self.run | self.step
    }
}

pub type InputPin<const P: char, const N: u8> = gpio::Pin<P, N, gpio::Input<gpio::PullDown>>;

pub struct Pins {
    pub reset: InputPin<'E', 8>,
    pub run: InputPin<'E', 9>,
    pub step: InputPin<'E', 10>,
}

pub struct Keyboard {
    pins: Pins,
}

impl Keyboard {
    pub fn new(pins: Pins) -> Self {
        Self { pins }
    }

    pub fn read_keys(&self) -> Keys {
        Keys {
            reset: self.is_reset_pressed(),
            run: self.is_run_pressed(),
            step: self.is_step_pressed(),
        }
    }

    pub fn is_reset_pressed(&self) -> bool {
        self.pins.reset.is_low()
    }

    pub fn is_run_pressed(&self) -> bool {
        self.pins.run.is_low()
    }

    pub fn is_step_pressed(&self) -> bool {
        self.pins.step.is_low()
    }
}
