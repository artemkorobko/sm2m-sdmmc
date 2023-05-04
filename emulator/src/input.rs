use stm32f1xx_hal::{device, gpio};

pub type InputPin<const P: char, const N: u8> = gpio::Pin<P, N, gpio::Input<gpio::PullDown>>;
pub type InputPinE<const N: u8> = InputPin<'E', N>;
pub type InputPinA<const N: u8> = InputPin<'A', N>;

pub struct GPIOAPins {
    pub ctrlo_0: InputPinA<6>,
    pub ctrlo_1: InputPinA<7>,
    pub ctrld: InputPinA<8>,
    pub erro: InputPinA<9>,
    pub rste: InputPinA<10>,
    pub sete: InputPinA<11>,
    pub dteo: InputPinA<12>,
}

pub struct GPIOEPins {
    pub do_0: InputPinE<0>,
    pub do_1: InputPinE<1>,
    pub do_2: InputPinE<2>,
    pub do_3: InputPinE<3>,
    pub do_4: InputPinE<4>,
    pub do_5: InputPinE<5>,
    pub do_6: InputPinE<6>,
    pub do_7: InputPinE<7>,
    pub do_8: InputPinE<8>,
    pub do_9: InputPinE<9>,
    pub do_10: InputPinE<10>,
    pub do_11: InputPinE<11>,
    pub do_12: InputPinE<12>,
    pub do_13: InputPinE<13>,
    pub do_14: InputPinE<14>,
    pub do_15: InputPinE<15>,
}

pub struct Bus {
    gpioa: device::GPIOA,
    gpioe: device::GPIOE,
}

impl Bus {
    pub fn new(_: GPIOAPins, _: GPIOEPins) -> Self {
        let periph = unsafe { device::Peripherals::steal() };

        Self {
            gpioa: periph.GPIOA,
            gpioe: periph.GPIOE,
        }
    }
}
