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
    file: &str,
    file_pos: &mut u32,
    buf: &mut Vec<u8>,
    buf_pos: &mut usize,
    led: &mut L,
    card: &mut sdmmc::Card,
) -> Result<Complete, AppError>
where
    L: OutputPin,
{
    // // received data transfer end signal
    // if input.dtei {
    //     led.set_high().ok(); // turn read LED off
    //     Ok(Complete::Mode(Mode::Ready))
    // } else {
    //     if buf.is_empty() || *buf_pos >= buf.len() {
    //         buf.clear();
    //         let mut ctl = card.open()?;
    //         let mut file = ctl.oped_file_read(file)?;
    //         file.seek_from_start(*file_pos)?;

    //         while buf.len() < buf.capacity() {
    //             let mut slice = [0; 1024];
    //             let size_to_read = core::cmp::min(slice.len(), buf.capacity() - buf.len());
    //             let size_read = ctl.read(&mut file, &mut slice[..size_to_read])?;
    //             buf.extend_from_slice(&slice[..size_read]);
    //             *file_pos += size_read as u32;
    //         }

    //         *buf_pos = 0;
    //     }

    //     match read_word(buf, *buf_pos) {
    //         Some(word) => {
    //             *buf_pos += 2;
    //             Ok(Complete::Reply(word))
    //         }
    //         None => Ok(Complete::Reply(0)),
    //     }
    // }

    Ok(Complete::Continue)
}

fn read_word(buf: &mut [u8], pos: usize) -> Option<u16> {
    buf.chunks(2).nth(pos).map(map_word)
}

fn map_word(chunk: &[u8]) -> u16 {
    match chunk {
        [hi, lo] => u16::from_be_bytes([*hi, *lo]),
        [hi] => u16::from_be_bytes([*hi, 0]),
        _ => 0,
    }
}
