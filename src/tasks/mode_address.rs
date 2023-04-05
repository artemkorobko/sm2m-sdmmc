use embedded_hal::digital::v2::OutputPin;

use crate::{
    mode::{AppError, Mode},
    peripherals::{sdmmc, sm2m},
};

pub fn handle<WL, RL>(
    input: sm2m::Input,
    write_led: &mut WL,
    read_led: &mut RL,
    card: &mut sdmmc::Card,
    file_name: &sdmmc::FileName,
) -> Result<Option<Mode>, AppError>
where
    WL: OutputPin,
    RL: OutputPin,
{
    let command = sm2m::Command::from(input).ok_or(AppError::UnknownCommand)?;
    match command {
        sm2m::Command::Write => write_command(write_led, card, file_name),
        sm2m::Command::Read => read_command(read_led, card, file_name),
        _ => super::command::cmd_unhandled(),
    }
}

fn write_command<L>(
    led: &mut L,
    card: &mut sdmmc::Card,
    file_name: &sdmmc::FileName,
) -> Result<Option<Mode>, AppError>
where
    L: OutputPin,
{
    // delette .bak file if exists
    let (mut controller, volume) = super::command::open_sdmmc(card)?;
    let dir = controller.open_root_dir(&volume)?;
    delete_file(&mut controller, &volume, &dir, &file_name)?;
    // copy original file to .bak
    // delete original file
    led.set_low().ok();
    Ok(None)
}

fn delete_file<'a>(
    controller: &mut sdmmc::card::Controller<'a>,
    volume: &embedded_sdmmc::Volume,
    dir: &embedded_sdmmc::Directory,
    file_name: &str,
) -> Result<bool, AppError> {
    match controller.delete_file_in_dir(volume, dir, file_name) {
        Ok(_) => Ok(true),
        Err(embedded_sdmmc::Error::FileNotFound) => Ok(false),
        Err(err) => Err(err.into()),
    }
}

fn copy_file(
    controller: &mut sdmmc::card::Controller,
    volume: &embedded_sdmmc::Volume,
    dir: &embedded_sdmmc::Directory,
    from_file_name: &str,
    to_file_name: &str,
) {
}

fn read_command<L>(
    led: &mut L,
    card: &mut sdmmc::Card,
    file_name: &sdmmc::FileName,
) -> Result<Option<Mode>, AppError>
where
    L: OutputPin,
{
    let (mut controller, volume) = super::command::open_sdmmc(card)?;
    let dir = controller.open_root_dir(&volume)?;
    if is_file_exists(&mut controller, &volume, &dir, file_name)? {
        let file_name = file_name.clone();
        led.set_low().ok();
        Ok(Some(Mode::Read(file_name, heapless::Vec::new())))
    } else {
        let err = embedded_sdmmc::Error::FileNotFound;
        Err(AppError::SdMmcController(err))
    }
}

fn is_file_exists<'a>(
    controller: &mut sdmmc::card::Controller<'a>,
    volume: &embedded_sdmmc::Volume,
    dir: &embedded_sdmmc::Directory,
    file_name: &str,
) -> Result<bool, AppError> {
    match controller.find_directory_entry(volume, dir, file_name) {
        Ok(_) => Ok(true),
        Err(embedded_sdmmc::Error::FileNotFound) => Ok(false),
        Err(err) => Err(err.into()),
    }
}
