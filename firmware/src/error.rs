use embedded_sdmmc::sdmmc::Error as SpiError;
use embedded_sdmmc::Error as SdMmcControllerError;
use embedded_sdmmc::FilenameError;

pub type ControllerError = SdMmcControllerError<SpiError>;

#[derive(Clone)]
pub enum AppError {
    SdmmcDetached,
    UnknownCommand,
    UnhandledCommand,
    SdMmcSpi(SpiError),
    SdMmcController(ControllerError),
    SdMmcFile(embedded_sdmmc::filesystem::FileError),
}

impl AppError {
    pub fn opcode(&self) -> u16 {
        use AppError::*;

        match self {
            SdmmcDetached => 1,
            UnknownCommand => 2,
            UnhandledCommand => 3,
            SdMmcController(ControllerError::DeviceError(SpiError::Transport))
            | SdMmcSpi(SpiError::Transport) => 4,
            SdMmcController(ControllerError::DeviceError(SpiError::CantEnableCRC))
            | SdMmcSpi(SpiError::CantEnableCRC) => 5,
            SdMmcController(ControllerError::DeviceError(SpiError::TimeoutReadBuffer))
            | SdMmcSpi(SpiError::TimeoutReadBuffer) => 6,
            SdMmcController(ControllerError::DeviceError(SpiError::TimeoutWaitNotBusy))
            | SdMmcSpi(SpiError::TimeoutWaitNotBusy) => 7,
            SdMmcController(ControllerError::DeviceError(SpiError::TimeoutCommand(_)))
            | SdMmcSpi(SpiError::TimeoutCommand(_)) => 8,
            SdMmcController(ControllerError::DeviceError(SpiError::TimeoutACommand(_)))
            | SdMmcSpi(SpiError::TimeoutACommand(_)) => 9,
            SdMmcController(ControllerError::DeviceError(SpiError::Cmd58Error))
            | SdMmcSpi(SpiError::Cmd58Error) => 10,
            SdMmcController(ControllerError::DeviceError(SpiError::RegisterReadError))
            | SdMmcSpi(SpiError::RegisterReadError) => 11,
            SdMmcController(ControllerError::DeviceError(SpiError::CrcError(_, _)))
            | SdMmcSpi(SpiError::CrcError(_, _)) => 12,
            SdMmcController(ControllerError::DeviceError(SpiError::ReadError))
            | SdMmcSpi(SpiError::ReadError) => 13,
            SdMmcController(ControllerError::DeviceError(SpiError::WriteError))
            | SdMmcSpi(SpiError::WriteError) => 14,
            SdMmcController(ControllerError::DeviceError(SpiError::BadState))
            | SdMmcSpi(SpiError::BadState) => 15,
            SdMmcController(ControllerError::DeviceError(SpiError::CardNotFound))
            | SdMmcSpi(SpiError::CardNotFound) => 16,
            SdMmcController(ControllerError::DeviceError(SpiError::GpioError))
            | SdMmcSpi(SpiError::GpioError) => 17,
            SdMmcController(ControllerError::FormatError(_)) => 18,
            SdMmcController(ControllerError::NoSuchVolume) => 19,
            SdMmcController(ControllerError::FilenameError(FilenameError::InvalidCharacter)) => 20,
            SdMmcController(ControllerError::FilenameError(FilenameError::FilenameEmpty)) => 21,
            SdMmcController(ControllerError::FilenameError(FilenameError::NameTooLong)) => 22,
            SdMmcController(ControllerError::FilenameError(FilenameError::MisplacedPeriod)) => 23,
            SdMmcController(ControllerError::FilenameError(FilenameError::Utf8Error)) => 24,
            SdMmcController(ControllerError::TooManyOpenDirs) => 25,
            SdMmcController(ControllerError::TooManyOpenFiles) => 26,
            SdMmcController(ControllerError::FileNotFound) => 27,
            SdMmcController(ControllerError::FileAlreadyOpen) => 28,
            SdMmcController(ControllerError::DirAlreadyOpen) => 29,
            SdMmcController(ControllerError::OpenedDirAsFile) => 30,
            SdMmcController(ControllerError::DeleteDirAsFile) => 31,
            SdMmcController(ControllerError::FileIsOpen) => 32,
            SdMmcController(ControllerError::Unsupported) => 33,
            SdMmcController(ControllerError::EndOfFile) => 34,
            SdMmcController(ControllerError::BadCluster) => 35,
            SdMmcController(ControllerError::ConversionError) => 36,
            SdMmcController(ControllerError::NotEnoughSpace) => 37,
            SdMmcController(ControllerError::AllocationError) => 38,
            SdMmcController(ControllerError::JumpedFree) => 39,
            SdMmcController(ControllerError::ReadOnly) => 40,
            SdMmcController(ControllerError::FileAlreadyExists) => 41,
            SdMmcController(ControllerError::BadBlockSize(_)) => 42,
            SdMmcController(ControllerError::NotInBlock) => 43,
            SdMmcFile(embedded_sdmmc::filesystem::FileError::InvalidOffset) => 44,
        }
    }
}

impl From<SpiError> for AppError {
    fn from(value: SpiError) -> Self {
        Self::SdMmcSpi(value)
    }
}

impl From<ControllerError> for AppError {
    fn from(value: ControllerError) -> Self {
        Self::SdMmcController(value)
    }
}

impl From<embedded_sdmmc::filesystem::FileError> for AppError {
    fn from(value: embedded_sdmmc::filesystem::FileError) -> Self {
        Self::SdMmcFile(value)
    }
}
