use stm32f1xx_hal::gpio::ExtiPin;

use crate::{
    app,
    mode::{AppError, Mode},
    peripherals::{
        sdmmc,
        sm2m::{self, Read, Write},
    },
};

use super::{mode_address, mode_read, mode_ready, mode_write};

pub enum Complete {
    Continue,
    Mode(Mode),
    Reply(u16),
}

pub fn command(cx: app::command::Context) {
    let mode = cx.local.state;
    let error_led = cx.local.error_led;
    let write_led = cx.local.write_led;
    let read_led = cx.local.read_led;
    let sdmmc_detect = cx.local.sdmmc_detect_pin;
    let card = cx.local.card;
    let input = unsafe { cx.local.input_bus.read() };
    let out_bus = cx.local.output_bus;
    let result = match mode {
        Mode::Ready => mode_ready::handle(input, error_led, sdmmc_detect),
        Mode::Address(file) => mode_address::handle(input, write_led, read_led, card, file),
        Mode::Write(file, buf) => mode_write::handle(input, file, buf, card),
        Mode::Read(file, buf, pos) => mode_read::handle(input, file, buf, *pos, card, out_bus),
        Mode::Error(err) => mode_error(err, out_bus),
    };

    match result {
        Ok(Complete::Mode(new)) => {
            *mode = new;
            unsafe { out_bus.write(&sm2m::Output::ok()) };
        }
        Ok(Complete::Continue) => unsafe { out_bus.write(&sm2m::Output::ok()) },
        Ok(Complete::Reply(data)) => unsafe { out_bus.write(&sm2m::Output::data(data)) },
        Err(err) => {
            error_led.set_low();
            unsafe { out_bus.write(&sm2m::Output::error()) };
            *mode = Mode::Error(err);
        }
    }

    cx.local.trigger.clear_interrupt_pending_bit();
}

fn mode_error<W: sm2m::Write>(err: &AppError, out_bus: &mut W) -> Result<Complete, AppError> {
    unsafe { out_bus.write(&sm2m::Output::data(err.opcode())) }
    Ok(Complete::Mode(Mode::Ready))
}

pub fn cmd_unhandled() -> Result<Complete, AppError> {
    Ok(Complete::Mode(Mode::Error(AppError::UnhandledCommand)))
}

// try to implement the same method on Card
pub fn open_sdmmc<'a>(
    card: &'a mut sdmmc::Card,
) -> Result<(sdmmc::card::Controller<'a>, embedded_sdmmc::Volume), AppError> {
    let block_spi = card.acquire()?;
    let time_source = sdmmc::StaticTimeSource::default();
    let mut controller = embedded_sdmmc::Controller::new(block_spi, time_source);
    let volume = controller.get_volume(embedded_sdmmc::VolumeIdx(0))?;
    Ok((controller, volume))
}
