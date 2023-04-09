use alloc::vec::Vec;
use embedded_hal::digital::v2::OutputPin;

use crate::{
    error::AppError,
    mode::Mode,
    peripherals::{sdmmc, sm2m},
};

use super::command::Complete;

pub fn handle<L, W>(
    input: sm2m::Input,
    file: &str,
    buf: &mut Vec<u8>,
    pos: &mut usize,
    led: &mut L,
    card: &mut sdmmc::Card,
    output_bus: &mut W,
) -> Result<Complete, AppError>
where
    L: OutputPin,
    W: sm2m::Write,
{
    if input.dtei {
        led.set_high().ok(); // turn read LED off
        Ok(Complete::Mode(Mode::Ready))
    } else {
        if buf.is_empty() {
            // let mut ctl = card.open()?;
            // let mut file = ctl.oped_file_read(file)?;
            // ctl.try_read_all_from(&mut file, *pos as u32, buf)?;
            *pos = 0;
        }

        // write data to bus
        // let low = buf.get(*pos).unwrap();
        // let high = buf.get(*pos).unwrap();
        // let data = low | (high << 8);
        // output_bus.write(&sm2m::Output::data(data));

        Ok(Complete::Continue)
    }
}
