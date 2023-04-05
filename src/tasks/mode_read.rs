use alloc::vec::Vec;

use crate::{
    mode::{AppError, Mode},
    peripherals::{sdmmc, sm2m},
};

pub fn handle<W>(
    input: sm2m::Input,
    buf: &mut Vec<u8>,
    card: &mut sdmmc::Card,
    output_bus: &mut W,
) -> Result<Option<Mode>, AppError>
where
    W: sm2m::Write,
{
    Ok(None)
}
