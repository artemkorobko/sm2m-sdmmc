use core::ops::{Deref, DerefMut};

use stm32f1xx_hal::{gpio, pac, spi};

use super::StaticTimeSource;

pub type Cs = gpio::PA4<gpio::Output>;
pub type Sck = gpio::PA5<gpio::Alternate>;
pub type Miso = gpio::PA6;
pub type Mosi = gpio::PA7<gpio::Alternate>;
pub type SpiPins = (Sck, Miso, Mosi);
pub type SpiBus = spi::Spi<pac::SPI1, spi::Spi1NoRemap, SpiPins, u8>;
pub type SdMmcSpi = embedded_sdmmc::SdMmcSpi<SpiBus, Cs>;
pub type BlockSpi<'a> = embedded_sdmmc::BlockSpi<'a, SpiBus, Cs>;
pub type Controller<'a> = embedded_sdmmc::Controller<BlockSpi<'a>, StaticTimeSource>;

pub struct Card(SdMmcSpi);

impl From<SdMmcSpi> for Card {
    fn from(value: SdMmcSpi) -> Self {
        Self(value)
    }
}

impl Deref for Card {
    type Target = SdMmcSpi;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Card {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
