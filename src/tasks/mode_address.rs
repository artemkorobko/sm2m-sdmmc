use alloc::{borrow::ToOwned, format, vec::Vec};
use embedded_hal::digital::v2::OutputPin;

use crate::{
    error::AppError,
    mode::Mode,
    peripherals::{sdmmc, sm2m},
};

use super::command::Complete;

pub fn handle<W, R>(
    input: sm2m::Input,
    write_led: &mut W,
    read_led: &mut R,
    card: &mut sdmmc::Card,
    file_name: &str,
) -> Result<Complete, AppError>
where
    W: OutputPin,
    R: OutputPin,
{
    match sm2m::Command::from(input)? {
        sm2m::Command::Write => write(write_led, card, file_name),
        sm2m::Command::Read => read(read_led, card, file_name),
        _ => Err(AppError::UnhandledCommand),
    }
}

const IO_BUFFER_CAPACITY: usize = 1024 * 10;

fn write<L>(led: &mut L, card: &mut sdmmc::Card, file: &str) -> Result<Complete, AppError>
where
    L: OutputPin,
{
    let mut ctl = card.open()?;
    if ctl.is_file_exists(file)? {
        ctl.copy_file(file, &format!("{file}.bak"))?;
        ctl.delete_file(file)?;
    }

    led.set_low().ok(); // turn write LED on
    Ok(Complete::Mode(Mode::Write(
        file.to_owned(),
        Vec::with_capacity(IO_BUFFER_CAPACITY),
    )))
}

fn read<L>(led: &mut L, card: &mut sdmmc::Card, file: &str) -> Result<Complete, AppError>
where
    L: OutputPin,
{
    let mut ctl = card.open()?;
    if ctl.is_file_exists(file)? {
        led.set_low().ok(); // turn read LED on
        Ok(Complete::Mode(Mode::Read(
            file.to_owned(),
            Vec::with_capacity(IO_BUFFER_CAPACITY),
            0,
        )))
    } else {
        Err(AppError::SdMmcController(
            embedded_sdmmc::Error::FileNotFound,
        ))
    }
}
