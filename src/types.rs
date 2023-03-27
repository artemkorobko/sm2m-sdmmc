pub mod sdmmc {
    use stm32f1xx_hal::{gpio, pac, spi};

    use crate::peripherals::led::Led;

    pub type DetectPin = gpio::PE0<gpio::Input<gpio::PullUp>>;
    pub type DetectLedPin = gpio::PD7<gpio::Output>;
    pub type DetectLed = Led<DetectLedPin>;
    pub type CS = gpio::PA4<gpio::Output>;
    pub type SCK = gpio::PC10<gpio::Alternate>;
    pub type MISO = gpio::PC11;
    pub type MOSI = gpio::PC12<gpio::Alternate>;
    pub type SpiPins = (SCK, MISO, MOSI);
    pub type SpiBus = spi::Spi<pac::SPI3, spi::Spi3Remap, SpiPins, u8>;
    pub type Bus = embedded_sdmmc::SdMmcSpi<SpiBus, CS>;

    pub const BUFFER_SIZE: usize = 8192;
    pub type Buffer = heapless::Vec<u8, BUFFER_SIZE>;
}

pub mod status {
    use stm32f1xx_hal::gpio;

    use crate::peripherals::status;

    pub type StatusLed1Pin = gpio::PC2<gpio::Output>;
    pub type StatusLed2Pin = gpio::PC0<gpio::Output>;
    pub type StatusLed3Pin = gpio::PC3<gpio::Output>;
    pub type StatusLed4Pin = gpio::PA3<gpio::Output>;
    pub type StatusLed5Pin = gpio::PA0<gpio::Output>;
    pub type StatusLed6Pin = gpio::PA6<gpio::Output>;

    pub type Display = status::Display<
        StatusLed1Pin,
        StatusLed2Pin,
        StatusLed3Pin,
        StatusLed4Pin,
        StatusLed5Pin,
        StatusLed6Pin,
    >;
}

pub mod sm2m {
    use stm32f1xx_hal::gpio;

    use crate::peripherals;

    pub type OutputBusBit0Pin = gpio::PE14<gpio::Output>;
    pub type OutputBusBit1Pin = gpio::PA11<gpio::Output>;
    pub type OutputBusBit2Pin = gpio::PA9<gpio::Output>;
    pub type OutputBusBit3Pin = gpio::PC9<gpio::Output>;
    pub type OutputBusBit4Pin = gpio::PC7<gpio::Output>;
    pub type OutputBusBit5Pin = gpio::PD15<gpio::Output>;
    pub type OutputBusBit6Pin = gpio::PD13<gpio::Output>;
    pub type OutputBusBit7Pin = gpio::PD11<gpio::Output>;
    pub type OutputBusBit8Pin = gpio::PD0<gpio::Output>;
    pub type OutputBusBit9Pin = gpio::PA12<gpio::Output>;
    pub type OutputBusBit10Pin = gpio::PA10<gpio::Output>;
    pub type OutputBusBit11Pin = gpio::PA8<gpio::Output>;
    pub type OutputBusBit12Pin = gpio::PC8<gpio::Output>;
    pub type OutputBusBit13Pin = gpio::PC6<gpio::Output>;
    pub type OutputBusBit14Pin = gpio::PD14<gpio::Output>;
    pub type OutputBusBit15Pin = gpio::PD12<gpio::Output>;

    pub type OutputReserved1 = gpio::PB14<gpio::Output>; // КРО
    pub type OutputReserved2 = gpio::PB15<gpio::Output>; // КР1
    pub type OutputReserved3 = gpio::PB10<gpio::Output>; // ВНУ
    pub type OutputReserved4 = gpio::PE12<gpio::Output>; // ГТ-П
    pub type OutputReserved5 = gpio::PE11<gpio::Output>; // ВНС

    pub type OutputBus = peripherals::sm2m::OutputBus<
        OutputBusBit0Pin,
        OutputBusBit1Pin,
        OutputBusBit2Pin,
        OutputBusBit3Pin,
        OutputBusBit4Pin,
        OutputBusBit5Pin,
        OutputBusBit6Pin,
        OutputBusBit7Pin,
        OutputBusBit8Pin,
        OutputBusBit9Pin,
        OutputBusBit10Pin,
        OutputBusBit11Pin,
        OutputBusBit12Pin,
        OutputBusBit13Pin,
        OutputBusBit14Pin,
        OutputBusBit15Pin,
    >;

    pub type InputBusBit0Pin = gpio::PB5<gpio::Input<gpio::PullDown>>;
    pub type InputBusBit1Pin = gpio::PB8<gpio::Input<gpio::PullDown>>;
    pub type InputBusBit2Pin = gpio::PE1<gpio::Input<gpio::PullDown>>;
    pub type InputBusBit3Pin = gpio::PE3<gpio::Input<gpio::PullDown>>;
    pub type InputBusBit4Pin = gpio::PE5<gpio::Input<gpio::PullDown>>;
    pub type InputBusBit5Pin = gpio::PE7<gpio::Input<gpio::PullDown>>;
    pub type InputBusBit6Pin = gpio::PE9<gpio::Input<gpio::PullDown>>;
    pub type InputBusBit7Pin = gpio::PD1<gpio::Input<gpio::PullDown>>;
    pub type InputBusBit8Pin = gpio::PB6<gpio::Input<gpio::PullDown>>;
    pub type InputBusBit9Pin = gpio::PB7<gpio::Input<gpio::PullDown>>;
    pub type InputBusBit10Pin = gpio::PB9<gpio::Input<gpio::PullDown>>;
    pub type InputBusBit11Pin = gpio::PE2<gpio::Input<gpio::PullDown>>;
    pub type InputBusBit12Pin = gpio::PE4<gpio::Input<gpio::PullDown>>;
    pub type InputBusBit13Pin = gpio::PE6<gpio::Input<gpio::PullDown>>;
    pub type InputBusBit14Pin = gpio::PE8<gpio::Input<gpio::PullDown>>;
    pub type InputBusBit15Pin = gpio::PE10<gpio::Input<gpio::PullDown>>;

    pub type InputReserved1 = gpio::PD2<gpio::Output>; // КРО
    pub type InputReserved2 = gpio::PD3<gpio::Output>; // КР1
    pub type InputReserved3 = gpio::PD4<gpio::Output>; // ВП-И
    pub type InputReserved4 = gpio::PD5<gpio::Output>; // ВПИП
    pub type InputReserved5 = gpio::PD7<gpio::Output>; // ОСТ-ИП
    pub type InputReserved6 = gpio::PD6<gpio::Output>; // ОСБ-И

    pub type InputBus = peripherals::sm2m::InputBus<
        InputBusBit0Pin,
        InputBusBit1Pin,
        InputBusBit2Pin,
        InputBusBit3Pin,
        InputBusBit4Pin,
        InputBusBit5Pin,
        InputBusBit6Pin,
        InputBusBit7Pin,
        InputBusBit8Pin,
        InputBusBit9Pin,
        InputBusBit10Pin,
        InputBusBit11Pin,
        InputBusBit12Pin,
        InputBusBit13Pin,
        InputBusBit14Pin,
        InputBusBit15Pin,
    >;
}
