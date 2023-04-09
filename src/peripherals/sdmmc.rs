pub mod card;
pub mod controller;
pub mod file;
pub mod time;

pub use card::Card;
pub use controller::Controller;
pub use file::AsFileName;
pub use time::StaticTimeSource;

use self::card::{Cs, SpiBus};

pub type SdMmcSpi<'a> = embedded_sdmmc::BlockSpi<'a, SpiBus, Cs>;
pub type SdMmcController<'a> = embedded_sdmmc::Controller<SdMmcSpi<'a>, StaticTimeSource>;
pub type SdMmcVolume = embedded_sdmmc::Volume;
pub type SdMmcDirectory = embedded_sdmmc::Directory;
pub type SdMmcFile = embedded_sdmmc::File;
