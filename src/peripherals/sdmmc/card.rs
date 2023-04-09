use stm32f1xx_hal::{gpio, pac, spi};

use crate::error::AppError;

use super::{Controller, StaticTimeSource};

pub type Cs = gpio::PA4<gpio::Output>;
pub type Sck = gpio::PA5<gpio::Alternate>;
pub type Miso = gpio::PA6;
pub type Mosi = gpio::PA7<gpio::Alternate>;
pub type SpiPins = (Sck, Miso, Mosi);
pub type SpiBus = spi::Spi<pac::SPI1, spi::Spi1NoRemap, SpiPins, u8>;
pub type SdMmcSpi = embedded_sdmmc::SdMmcSpi<SpiBus, Cs>;

pub struct Card(SdMmcSpi);

impl Card {
    pub fn open(&mut self) -> Result<Controller, AppError> {
        let spi = self.0.acquire()?;
        let time = StaticTimeSource::default();
        let mut ctl = embedded_sdmmc::Controller::new(spi, time);
        let vol = ctl.get_volume(embedded_sdmmc::VolumeIdx(0))?;
        let dir = ctl.open_root_dir(&vol)?;
        Ok(Controller::new(ctl, vol, dir))
    }
}

impl From<SdMmcSpi> for Card {
    fn from(value: SdMmcSpi) -> Self {
        Self(value)
    }
}
