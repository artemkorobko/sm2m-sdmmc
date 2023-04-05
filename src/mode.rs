use alloc::{string::String, vec::Vec};
use embedded_sdmmc::sdmmc;

#[derive(Clone)]
pub enum AppError {
    SdmmcDetached,
    UnknownCommand,
    UnhandledCommand,
    SdMmcSpi(sdmmc::Error),
    SdMmcController(embedded_sdmmc::Error<embedded_sdmmc::sdmmc::Error>),
}

impl AppError {
    pub fn opcode(&self) -> u16 {
        match self {
            AppError::SdmmcDetached => 1,
            AppError::UnknownCommand => 2,
            AppError::UnhandledCommand => 3,
            AppError::SdMmcController(embedded_sdmmc::Error::DeviceError(
                sdmmc::Error::Transport,
            ))
            | AppError::SdMmcSpi(sdmmc::Error::Transport) => 4,
            AppError::SdMmcController(embedded_sdmmc::Error::DeviceError(
                sdmmc::Error::CantEnableCRC,
            ))
            | AppError::SdMmcSpi(sdmmc::Error::CantEnableCRC) => 5,
            AppError::SdMmcController(embedded_sdmmc::Error::DeviceError(
                sdmmc::Error::TimeoutReadBuffer,
            ))
            | AppError::SdMmcSpi(sdmmc::Error::TimeoutReadBuffer) => 6,
            AppError::SdMmcController(embedded_sdmmc::Error::DeviceError(
                sdmmc::Error::TimeoutWaitNotBusy,
            ))
            | AppError::SdMmcSpi(sdmmc::Error::TimeoutWaitNotBusy) => 7,
            AppError::SdMmcController(embedded_sdmmc::Error::DeviceError(
                sdmmc::Error::TimeoutCommand(_),
            ))
            | AppError::SdMmcSpi(sdmmc::Error::TimeoutCommand(_)) => 8,
            AppError::SdMmcController(embedded_sdmmc::Error::DeviceError(
                sdmmc::Error::TimeoutACommand(_),
            ))
            | AppError::SdMmcSpi(sdmmc::Error::TimeoutACommand(_)) => 9,
            AppError::SdMmcController(embedded_sdmmc::Error::DeviceError(
                sdmmc::Error::Cmd58Error,
            ))
            | AppError::SdMmcSpi(sdmmc::Error::Cmd58Error) => 10,
            AppError::SdMmcController(embedded_sdmmc::Error::DeviceError(
                sdmmc::Error::RegisterReadError,
            ))
            | AppError::SdMmcSpi(sdmmc::Error::RegisterReadError) => 11,
            AppError::SdMmcController(embedded_sdmmc::Error::DeviceError(
                sdmmc::Error::CrcError(_, _),
            ))
            | AppError::SdMmcSpi(sdmmc::Error::CrcError(_, _)) => 12,
            AppError::SdMmcController(embedded_sdmmc::Error::DeviceError(
                sdmmc::Error::ReadError,
            ))
            | AppError::SdMmcSpi(sdmmc::Error::ReadError) => 13,
            AppError::SdMmcController(embedded_sdmmc::Error::DeviceError(
                sdmmc::Error::WriteError,
            ))
            | AppError::SdMmcSpi(sdmmc::Error::WriteError) => 14,
            AppError::SdMmcController(embedded_sdmmc::Error::DeviceError(
                sdmmc::Error::BadState,
            ))
            | AppError::SdMmcSpi(sdmmc::Error::BadState) => 15,
            AppError::SdMmcController(embedded_sdmmc::Error::DeviceError(
                sdmmc::Error::CardNotFound,
            ))
            | AppError::SdMmcSpi(sdmmc::Error::CardNotFound) => 16,
            AppError::SdMmcController(embedded_sdmmc::Error::DeviceError(
                sdmmc::Error::GpioError,
            ))
            | AppError::SdMmcSpi(sdmmc::Error::GpioError) => 17,
            AppError::SdMmcController(embedded_sdmmc::Error::FormatError(_)) => 18,
            AppError::SdMmcController(embedded_sdmmc::Error::NoSuchVolume) => 19,
            AppError::SdMmcController(embedded_sdmmc::Error::FilenameError(
                embedded_sdmmc::FilenameError::InvalidCharacter,
            )) => 20,
            AppError::SdMmcController(embedded_sdmmc::Error::FilenameError(
                embedded_sdmmc::FilenameError::FilenameEmpty,
            )) => 21,
            AppError::SdMmcController(embedded_sdmmc::Error::FilenameError(
                embedded_sdmmc::FilenameError::NameTooLong,
            )) => 22,
            AppError::SdMmcController(embedded_sdmmc::Error::FilenameError(
                embedded_sdmmc::FilenameError::MisplacedPeriod,
            )) => 23,
            AppError::SdMmcController(embedded_sdmmc::Error::FilenameError(
                embedded_sdmmc::FilenameError::Utf8Error,
            )) => 24,
            AppError::SdMmcController(embedded_sdmmc::Error::TooManyOpenDirs) => 25,
            AppError::SdMmcController(embedded_sdmmc::Error::TooManyOpenFiles) => 26,
            AppError::SdMmcController(embedded_sdmmc::Error::FileNotFound) => 27,
            AppError::SdMmcController(embedded_sdmmc::Error::FileAlreadyOpen) => 28,
            AppError::SdMmcController(embedded_sdmmc::Error::DirAlreadyOpen) => 29,
            AppError::SdMmcController(embedded_sdmmc::Error::OpenedDirAsFile) => 30,
            AppError::SdMmcController(embedded_sdmmc::Error::DeleteDirAsFile) => 31,
            AppError::SdMmcController(embedded_sdmmc::Error::FileIsOpen) => 32,
            AppError::SdMmcController(embedded_sdmmc::Error::Unsupported) => 33,
            AppError::SdMmcController(embedded_sdmmc::Error::EndOfFile) => 34,
            AppError::SdMmcController(embedded_sdmmc::Error::BadCluster) => 35,
            AppError::SdMmcController(embedded_sdmmc::Error::ConversionError) => 36,
            AppError::SdMmcController(embedded_sdmmc::Error::NotEnoughSpace) => 37,
            AppError::SdMmcController(embedded_sdmmc::Error::AllocationError) => 38,
            AppError::SdMmcController(embedded_sdmmc::Error::JumpedFree) => 39,
            AppError::SdMmcController(embedded_sdmmc::Error::ReadOnly) => 40,
            AppError::SdMmcController(embedded_sdmmc::Error::FileAlreadyExists) => 41,
            AppError::SdMmcController(embedded_sdmmc::Error::BadBlockSize(_)) => 42,
            AppError::SdMmcController(embedded_sdmmc::Error::NotInBlock) => 43,
        }
    }
}

impl From<sdmmc::Error> for AppError {
    fn from(value: sdmmc::Error) -> Self {
        Self::SdMmcSpi(value)
    }
}

impl From<embedded_sdmmc::Error<embedded_sdmmc::sdmmc::Error>> for AppError {
    fn from(value: embedded_sdmmc::Error<embedded_sdmmc::sdmmc::Error>) -> Self {
        Self::SdMmcController(value)
    }
}

pub enum Mode {
    Ready,
    Address(String),
    Write(String, Vec<u8>),
    Read(String, Vec<u8>),
    Error(AppError),
}
