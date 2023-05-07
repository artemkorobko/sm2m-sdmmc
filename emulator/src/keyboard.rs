use stm32f1xx_hal::gpio;

pub enum Key {
    StartWrite,
    StartRead,
    Step,
    Stop,
    Debug,
}

pub type InputPin<const P: char, const N: u8> = gpio::Pin<P, N, gpio::Input<gpio::PullDown>>;

pub struct Pins {
    pub start_write: InputPin<'E', 8>,
    pub start_read: InputPin<'E', 10>,
    pub step: InputPin<'E', 9>,
    pub stop: InputPin<'B', 2>,
    pub debug: InputPin<'E', 7>,
}

pub struct Keyboard {
    pins: Pins,
}

impl Keyboard {
    pub fn new(pins: Pins) -> Self {
        Self { pins }
    }

    pub fn read_key(&self) -> Option<Key> {
        if self.pins.start_write.is_low() {
            Some(Key::StartWrite)
        } else if self.pins.start_read.is_low() {
            Some(Key::StartRead)
        } else if self.pins.step.is_low() {
            Some(Key::Step)
        } else if self.pins.stop.is_low() {
            Some(Key::Stop)
        } else if self.pins.debug.is_low() {
            Some(Key::Debug)
        } else {
            None
        }
    }
}
