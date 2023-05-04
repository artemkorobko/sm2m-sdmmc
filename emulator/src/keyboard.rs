use stm32f1xx_hal::gpio;

pub struct Keys {
    pub reset: bool,
    pub step: bool,
}

impl Keys {
    pub fn is_pressed(&self) -> bool {
        self.reset | self.step
    }
}

pub type InputPin<const P: char, const N: u8> = gpio::Pin<P, N, gpio::Input<gpio::PullDown>>;

pub struct KeyboardPins {
    pub reset: InputPin<'B', 3>, // reset
    pub step: InputPin<'A', 3>,  // step
}

pub struct Keyboard {
    pins: KeyboardPins,
}

impl Keyboard {
    pub fn configure(pins: KeyboardPins) -> Self {
        Self { pins }
    }

    pub fn read_keys(&self) -> Keys {
        Keys {
            reset: self.is_reset_pressed(),
            step: self.is_step_pressed(),
        }
    }

    pub fn is_reset_pressed(&self) -> bool {
        self.pins.reset.is_low()
    }

    pub fn is_step_pressed(&self) -> bool {
        self.pins.step.is_low()
    }
}
