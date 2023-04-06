use alloc::vec::Vec;

use crate::{
    mode::{AppError, Mode},
    peripherals::{sdmmc, sm2m},
};

use super::command::Complete;

pub fn handle(
    input: sm2m::Input,
    file_name: &str,
    buf: &mut Vec<u8>,
    card: &mut sdmmc::Card,
) -> Result<Complete, AppError> {
    if input.dtei {
        Ok(Complete::Mode(Mode::Ready))
    } else {
        buf.extend_from_slice(&input.data.to_be_bytes());

        if buf.len() == buf.capacity() {
            let (mut controller, mut volume) = super::command::open_sdmmc(card)?;
            let dir = controller.open_root_dir(&volume)?;
            let mode = embedded_sdmmc::Mode::ReadWriteCreateOrAppend;
            let mut file = controller.open_file_in_dir(&mut volume, &dir, file_name, mode)?;
            write_entire_buf(&mut controller, &mut volume, &mut file, buf)?;
            controller.close_dir(&volume, dir);
        }

        Ok(Complete::Continue)
    }
}

fn write_entire_buf(
    controller: &mut sdmmc::card::Controller,
    volume: &mut embedded_sdmmc::Volume,
    file: &mut embedded_sdmmc::File,
    buf: &[u8],
) -> Result<usize, AppError> {
    let mut written = 0;
    while written < buf.len() {
        written += controller.write(volume, file, &buf[written..])?;
    }
    Ok(written)
}
