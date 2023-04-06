use embedded_hal::digital::v2::{InputPin, OutputPin};

use crate::{
    mode::{AppError, Mode},
    peripherals::{sdmmc, sm2m},
};

use super::command::Complete;

pub fn handle<L, P>(
    input: sm2m::Input,
    err_led: &mut L,
    sdmmc_detect: &P,
) -> Result<Complete, AppError>
where
    L: OutputPin,
    P: InputPin,
{
    let command = sm2m::Command::from(input).ok_or(AppError::UnknownCommand)?;
    match command {
        sm2m::Command::CheckStatus => cmd_check_status(sdmmc_detect),
        sm2m::Command::Reset => cmd_reset(err_led),
        sm2m::Command::Address(addr) => cmd_address(addr),
        _ => super::command::cmd_unhandled(),
    }
}

fn cmd_check_status<P>(sdmmc_detect: &P) -> Result<Complete, AppError>
where
    P: InputPin,
{
    if sdmmc_detect.is_low().unwrap_or(true) {
        Err(AppError::SdmmcDetached)
    } else {
        Ok(Complete::Continue)
    }
}

fn cmd_reset<L>(err_led: &mut L) -> Result<Complete, AppError>
where
    L: OutputPin,
{
    err_led.set_high().ok();
    Ok(Complete::Continue)
}

fn cmd_address(addr: u16) -> Result<Complete, AppError> {
    let file_name = sdmmc::AsFileName::as_file_name(&addr);
    Ok(Complete::Mode(Mode::Address(file_name)))
}
