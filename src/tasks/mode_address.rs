use alloc::{format, vec::Vec};
use embedded_hal::digital::v2::OutputPin;

use crate::{
    mode::{AppError, Mode},
    peripherals::{sdmmc, sm2m},
};

use super::command::Complete;

const IO_BUFFER_CAPACITY: usize = 1024 * 10;

pub fn handle<WL, RL>(
    input: sm2m::Input,
    write_led: &mut WL,
    read_led: &mut RL,
    card: &mut sdmmc::Card,
    file_name: &str,
) -> Result<Complete, AppError>
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
    file_name: &str,
) -> Result<Complete, AppError>
where
    L: OutputPin,
{
    let (mut controller, mut volume) = super::command::open_sdmmc(card)?;
    let dir = controller.open_root_dir(&volume)?;
    let bakup_file_file = format!("{}.bak", file_name);

    if is_file_exists(&mut controller, &volume, &dir, file_name)? {
        copy_file(
            &mut controller,
            &mut volume,
            &dir,
            &file_name,
            &bakup_file_file,
        )?;
        delete_file(&mut controller, &volume, &dir, file_name)?;
        controller.close_dir(&volume, dir);
    }

    led.set_low().ok();
    Ok(Complete::Mode(Mode::Write(
        file_name.into(),
        Vec::with_capacity(IO_BUFFER_CAPACITY),
    )))
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
    volume: &mut embedded_sdmmc::Volume,
    dir: &embedded_sdmmc::Directory,
    src_file_name: &str,
    dst_file_name: &str,
) -> Result<bool, AppError> {
    let mode = embedded_sdmmc::Mode::ReadOnly;
    let mut src = controller.open_file_in_dir(volume, dir, src_file_name, mode)?;
    let mode = embedded_sdmmc::Mode::ReadWriteCreateOrTruncate;
    let mut dst = controller.open_file_in_dir(volume, dir, dst_file_name, mode)?;
    let mut buf = [0; 64];
    loop {
        match controller.read(volume, &mut src, &mut buf) {
            Ok(size) => controller.write(volume, &mut dst, &buf[..size])?,
            Err(embedded_sdmmc::Error::EndOfFile) => return Ok(true),
            Err(err) => return Err(err.into()),
        };
    }
}

fn read_command<L>(
    led: &mut L,
    card: &mut sdmmc::Card,
    file_name: &str,
) -> Result<Complete, AppError>
where
    L: OutputPin,
{
    let (mut controller, volume) = super::command::open_sdmmc(card)?;
    let dir = controller.open_root_dir(&volume)?;
    if is_file_exists(&mut controller, &volume, &dir, file_name)? {
        let file_name = file_name.clone();
        led.set_low().ok();
        controller.close_dir(&volume, dir);
        Ok(Complete::Mode(Mode::Read(
            file_name.into(),
            Vec::with_capacity(IO_BUFFER_CAPACITY),
            0,
        )))
    } else {
        controller.close_dir(&volume, dir);
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
