use stm32f1xx_hal::{device, gpio};

pub type InputPin<const P: char, const N: u8> = gpio::Pin<P, N, gpio::Input<gpio::PullDown>>;

pub enum Frame {
    Confirm,
}

pub struct Pins {
    pub do_0: InputPin<'D', 0>,
    pub do_1: InputPin<'D', 1>,
    pub do_2: InputPin<'D', 2>,
    pub do_3: InputPin<'D', 3>,
    pub do_4: InputPin<'D', 4>,
    pub do_5: InputPin<'D', 5>,
    pub do_6: InputPin<'D', 6>,
    pub do_7: InputPin<'E', 11>,
    pub do_8: InputPin<'A', 8>,
    pub do_9: InputPin<'C', 6>,
    pub do_10: InputPin<'C', 7>,
    pub do_11: InputPin<'D', 11>,
    pub do_12: InputPin<'D', 12>,
    pub do_13: InputPin<'D', 13>,
    pub do_14: InputPin<'D', 14>,
    pub do_15: InputPin<'D', 15>,
    pub ctrlo_0: InputPin<'B', 15>,
    pub ctrlo_1: InputPin<'B', 14>,
    pub ctrld: InputPin<'B', 10>,
    pub erro: InputPin<'E', 15>,
    pub rste: InputPin<'E', 14>,
    pub sete: InputPin<'E', 13>,
    pub dteo: InputPin<'E', 12>,
}

pub struct Bus {
    gpioa: device::GPIOA,
    gpiob: device::GPIOB,
    gpioc: device::GPIOC,
    gpiod: device::GPIOD,
    gpioe: device::GPIOE,
}

impl Bus {
    pub fn new(_: Pins) -> Self {
        let periph = unsafe { device::Peripherals::steal() };

        Self {
            gpioa: periph.GPIOA,
            gpiob: periph.GPIOB,
            gpioc: periph.GPIOC,
            gpiod: periph.GPIOD,
            gpioe: periph.GPIOE,
        }
    }
}

struct GpioMask {
    pub gpioa: u16,
    pub gpiob: u16,
    pub gpioc: u16,
    pub gpioe: u16,
}

impl GpioMask {
    pub fn reverse_bits(self) -> Self {
        Self {
            gpioa: self.gpioa ^ u16::MAX,
            gpiob: self.gpiob ^ u16::MAX,
            gpioc: self.gpioc ^ u16::MAX,
            gpioe: self.gpioe ^ u16::MAX,
        }
    }
}

impl From<GpioMask> for Frame {
    fn from(value: GpioMask) -> Self {
        Self::Confirm
    }
}
