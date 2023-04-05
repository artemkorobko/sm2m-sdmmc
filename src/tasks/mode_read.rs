use alloc::vec::Vec;

use crate::{
    mode::{AppError, Mode},
    peripherals::{sdmmc, sm2m},
};

pub fn handle<W>(
    input: sm2m::Input,
    file_name: &str,
    buf: &mut Vec<u8>,
    pos: usize,
    card: &mut sdmmc::Card,
    output_bus: &mut W,
) -> Result<Option<Mode>, AppError>
where
    W: sm2m::Write,
{
    if buf.is_empty() {
        // fill buffer with data
        let (controller, volume) = super::command::open_sdmmc(card)?;
    }

    Ok(None)
}
