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
        led.set_high().ok(); // turn write LED off
        Ok(Complete::Mode(Mode::Ready))
    } else {
        buf.extend_from_slice(&input.data.to_be_bytes());
        if buf.len() == buf.capacity() {
            let mut ctl = card.open()?;
            let mut file = ctl.oped_file_append(file)?;
            ctl.write_all(&mut file, buf)?;
        }

        Ok(Complete::Continue)
    }
}
