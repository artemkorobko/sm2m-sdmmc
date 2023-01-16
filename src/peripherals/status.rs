use embedded_hal::digital::v2::OutputPin;

use super::led::Led;

pub struct Display<P1, P2, P3, P4, P5, P6> {
    pin1: Led<P1>,
    pin2: Led<P2>,
    pin3: Led<P3>,
    pin4: Led<P4>,
    pin5: Led<P5>,
    pin6: Led<P6>,
}

impl<P1: OutputPin, P2: OutputPin, P3: OutputPin, P4: OutputPin, P5: OutputPin, P6: OutputPin>
    Display<P1, P2, P3, P4, P5, P6>
{
    pub fn new(pin1: P1, pin2: P2, pin3: P3, pin4: P4, pin5: P5, pin6: P6) -> Self {
        Self {
            pin1: Led::new(pin1),
            pin2: Led::new(pin2),
            pin3: Led::new(pin3),
            pin4: Led::new(pin4),
            pin5: Led::new(pin5),
            pin6: Led::new(pin6),
        }
    }

    pub fn display(&mut self, value: u8) {
        write_bit(&mut self.pin1, value, 0);
        write_bit(&mut self.pin2, value, 1);
        write_bit(&mut self.pin3, value, 2);
        write_bit(&mut self.pin4, value, 3);
        write_bit(&mut self.pin5, value, 4);
        write_bit(&mut self.pin6, value, 5);
    }
}

fn write_bit<P: OutputPin>(pin: &mut Led<P>, value: u8, index: u8) {
    if value & 1 << index != 0 {
        pin.on();
    } else {
        pin.off();
    }
}
