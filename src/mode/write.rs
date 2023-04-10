use alloc::vec::Vec;
use embedded_hal::digital::v2::OutputPin;

use crate::{
    error::AppError,
    mode::Mode,
    peripherals::{sdmmc, sm2m},
};

use super::result::Complete;

pub fn handle<L>(
    input: sm2m::Input,
    led: &mut L,
    file: &str,
    buf: &mut Vec<u8>,
    card: &mut sdmmc::Card,
) -> Result<Complete, AppError>
where
    L: OutputPin,
{
    // received data transfer end signal
    if input.dtei {
        // save remaining buffer
        if !buf.is_empty() {
            save(buf, file, card)?;
        }

        led.set_high().ok(); // turn write LED off
        Ok(Complete::Mode(Mode::Ready))
    } else {
        buf.extend_from_slice(&input.data.to_be_bytes());
        if buf.len() == buf.capacity() {
            save(buf, file, card)?;
        }

        Ok(Complete::Continue)
    }
}

fn save(buf: &[u8], file: &str, card: &mut sdmmc::Card) -> Result<usize, AppError> {
    let mut ctl = card.open()?;
    let mut file = ctl.oped_file_append(file)?;
    let size = ctl.write(&mut file, buf)?;
    Ok(size)
}
