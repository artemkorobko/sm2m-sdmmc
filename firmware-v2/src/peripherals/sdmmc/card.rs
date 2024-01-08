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
pub type SdMmcDetectPin = gpio::PA3<gpio::Input<gpio::PullUp>>;

pub struct Card {
    spi: SdMmcSpi,
    _detect_pin: SdMmcDetectPin,
}

impl Card {
    pub fn new(spi: SdMmcSpi, detect_pin: SdMmcDetectPin) -> Self {
        Self {
            spi,
            _detect_pin: detect_pin,
        }
    }

    pub fn is_attached(&mut self) -> bool {
        match self.open() {
            Ok(controller) => {
                controller.close();
                true
            }
            Err(_) => false,
        }
    }

    pub fn open(&mut self) -> Result<Controller<'_>, AppError> {
        let spi = self.spi.acquire()?;
        let time = StaticTimeSource::default();
        let mut ctl = embedded_sdmmc::Controller::new(spi, time);
        let vol = ctl.get_volume(embedded_sdmmc::VolumeIdx(0))?;
        let dir = ctl.open_root_dir(&vol)?;
        Ok(Controller::new(ctl, vol, dir))
    }
}
