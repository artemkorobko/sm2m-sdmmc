use embedded_hal::digital::v2::{InputPin, OutputPin};

use crate::{
    error::AppError,
    mode::Mode,
    peripherals::{sdmmc, sm2m},
};

use super::result::Complete;

pub fn handle<EL, WL, RL, D>(
    input: sm2m::Input,
    err_led: &mut EL,
    write_led: &mut WL,
    read_led: &mut RL,
    sdmmc_detect: &D,
) -> Result<Complete, AppError>
where
    EL: OutputPin,
    WL: OutputPin,
    RL: OutputPin,
    D: InputPin,
{
    match sm2m::Command::from(input)? {
        sm2m::Command::CheckStatus => check_status(sdmmc_detect),
        sm2m::Command::Reset => reset(err_led, write_led, read_led),
        sm2m::Command::Address(addr) => address(addr),
        _ => Err(AppError::UnhandledCommand),
    }
}

fn check_status<P>(sdmmc_detect: &P) -> Result<Complete, AppError>
where
    P: InputPin,
{
    if sdmmc_detect.is_low().unwrap_or(true) {
        Err(AppError::SdmmcDetached)
    } else {
        Ok(Complete::Continue)
    }
}

fn reset<EL, WL, RL>(
    err_led: &mut EL,
    write_led: &mut WL,
    read_led: &mut RL,
) -> Result<Complete, AppError>
where
    EL: OutputPin,
    WL: OutputPin,
    RL: OutputPin,
{
    err_led.set_high().ok(); // turn error LED off
    write_led.set_high().ok(); // turn write LED off
    read_led.set_high().ok(); // turn read LED off
    Ok(Complete::Continue)
}

fn address(addr: u16) -> Result<Complete, AppError> {
    let file = sdmmc::AsFileName::as_file_name(&addr);
    Ok(Complete::Mode(Mode::Address(file)))
}
