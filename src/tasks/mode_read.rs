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
    pos: &mut usize,
    led: &mut L,
    card: &mut sdmmc::Card,
) -> Result<Complete, AppError>
where
    L: OutputPin,
{
    if input.dtei {
        led.set_high().ok(); // turn read LED off
        Ok(Complete::Mode(Mode::Ready))
    } else {
        if buf.is_empty() || buf.len() == *pos * 2 {
            let mut ctl = card.open()?;
            let mut file = ctl.oped_file_read(file)?;
            ctl.try_read_all_from(&mut file, *pos as u32, buf)?; // this can fail if buffer is empty
            *pos = 0;
        }

        match read_word(buf, *pos) {
            Some(word) => {
                *pos += 1;
                Ok(Complete::Reply(word))
            }
            None => Ok(Complete::Reply(0)),
        }
    }
}

fn read_word(buf: &mut Vec<u8>, pos: usize) -> Option<u16> {
    buf.chunks(2).skip(pos).next().map(map_word)
}

fn map_word(chunk: &[u8]) -> u16 {
    match chunk {
        &[hi, lo] => u16::from_be_bytes([hi, lo]),
        &[hi] => u16::from_be_bytes([hi, 0]),
        _ => 0,
    }
}
