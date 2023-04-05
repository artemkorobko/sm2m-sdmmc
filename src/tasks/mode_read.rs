use crate::{
    mode::{AppError, Mode},
    peripherals::sm2m,
};

pub fn handle<W>(input: sm2m::Input, output_bus: &mut W) -> Result<Option<Mode>, AppError>
where
    W: sm2m::Write,
{
    Ok(None)
}
