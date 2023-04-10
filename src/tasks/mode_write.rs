use alloc::vec::Vec;
use embedded_hal::digital::v2::OutputPin;

use crate::{
    error::AppError,
    mode::Mode,
    peripherals::{sdmmc, sm2m},
};

use super::command::Complete;

pub fn handle<L>(
    input: sm2m::Input,
    file: &str,
    buf: &mut Vec<u8>,
    led: &mut L,
    card: &mut sdmmc::Card,
) -> Result<Complete, AppError>
where
    L: OutputPin,
{
    if input.dtei {
        if !buf.is_empty() {
            dump_buf(buf, file, card)?;
        }

        led.set_high().ok(); // turn write LED off
        Ok(Complete::Mode(Mode::Ready))
    } else {
        buf.extend_from_slice(&input.data.to_be_bytes());
        if buf.len() == buf.capacity() {
            dump_buf(buf, file, card)?;
        }

        Ok(Complete::Continue)
    }
}

fn dump_buf(buf: &mut Vec<u8>, file: &str, card: &mut sdmmc::Card) -> Result<usize, AppError> {
    let mut ctl = card.open()?;
    let mut file = ctl.oped_file_append(file)?;
    let size = ctl.write_all(&mut file, buf)?;
    buf.clear();
    Ok(size)
}
