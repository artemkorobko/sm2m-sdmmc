use stm32f1xx_hal::{device, gpio};

pub type OutputPin<const P: char, const N: u8> = gpio::Pin<P, N, gpio::Output<gpio::PushPull>>;
pub type OutputPinD<const N: u8> = gpio::Pin<'D', N, gpio::Output<gpio::PushPull>>;
pub type OutputPinB<const N: u8> = gpio::Pin<'B', N, gpio::Output<gpio::PushPull>>;

pub struct GPIOBPins {
    pub ctrli_0: OutputPinB<0>, // CTRLI_0
    pub ctrli_1: OutputPinB<1>, // CTRLI_1
    pub dtsi: OutputPinB<4>,    // DTSI
    pub dtli: OutputPinB<5>,    // DTLI
    pub dtei: OutputPinB<6>,    // DTEI
    pub di_7: OutputPinB<7>,    // DI_7
    pub di_8: OutputPinB<8>,    // DI_8
    pub di_9: OutputPinB<9>,    // DI_9
    pub di_10: OutputPinB<10>,  // DI_10
    pub rsti: OutputPinB<14>,   // RSTI
}

pub struct GPIODPins {
    pub di_0: OutputPinD<0>,   // DI_0
    pub di_1: OutputPinD<1>,   // DI_1
    pub di_2: OutputPinD<2>,   // DI_2
    pub di_3: OutputPinD<3>,   // DI_3
    pub di_4: OutputPinD<4>,   // DI_4
    pub di_5: OutputPinD<5>,   // DI_5
    pub di_6: OutputPinD<6>,   // DI_6
    pub di_11: OutputPinD<11>, // DI_11
    pub di_12: OutputPinD<12>, // DI_12
    pub di_13: OutputPinD<13>, // DI_13
    pub di_14: OutputPinD<14>, // DI_14
    pub di_15: OutputPinD<15>, // DI_15
}

pub struct Bus {
    gpiob: device::GPIOB,
    gpiod: device::GPIOD,
}

impl Bus {
    pub fn new(_: GPIOBPins, _: GPIODPins) -> Self {
        let periph = unsafe { device::Peripherals::steal() };

        Self {
            gpiob: periph.GPIOB,
            gpiod: periph.GPIOD,
        }
    }
}
