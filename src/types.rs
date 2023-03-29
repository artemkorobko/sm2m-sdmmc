pub mod sdmmc {
    use stm32f1xx_hal::{gpio, pac, spi};

    use crate::peripherals::led::Led;

    pub type DetectPin = gpio::PA3<gpio::Input<gpio::PullUp>>;
    pub type DetectLedPin = gpio::PA0<gpio::Output>;
    pub type DetectLed = Led<DetectLedPin>;
    pub type CS = gpio::PA4<gpio::Output>;
    pub type SCK = gpio::PA5<gpio::Alternate>;
    pub type MISO = gpio::PA6;
    pub type MOSI = gpio::PA7<gpio::Alternate>;
    pub type SpiPins = (SCK, MISO, MOSI);
    pub type SpiBus = spi::Spi<pac::SPI1, spi::Spi1NoRemap, SpiPins, u8>;
    pub type Bus = embedded_sdmmc::SdMmcSpi<SpiBus, CS>;

    pub const BUFFER_SIZE: usize = 1024 * 10;
    pub type Buffer = heapless::Vec<u8, BUFFER_SIZE>;
}
