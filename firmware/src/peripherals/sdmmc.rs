pub mod card;
pub mod controller;
pub mod file;
pub mod time;

pub use card::Card;
pub use controller::Controller;
pub use file::{AsFileName, FileName, FileNameEx};
pub use time::StaticTimeSource;

use self::card::{Cs, SpiBus};

pub type SdMmcBlockSpi<'a> = embedded_sdmmc::BlockSpi<'a, SpiBus, Cs>;
pub type SdMmcController<'a> = embedded_sdmmc::Controller<SdMmcBlockSpi<'a>, StaticTimeSource>;
pub type SdMmcVolume = embedded_sdmmc::Volume;
pub type SdMmcDirectory = embedded_sdmmc::Directory;
pub type SdMmcFile = embedded_sdmmc::File;
