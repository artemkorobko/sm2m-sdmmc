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
        Mode::Write(file, buf) => mode_write::handle(input, buf, card, out_bus),
        Mode::Read(file, buf) => mode_read::handle(input, buf, card, out_bus),
        Mode::Error(err) => mode_error(err, out_bus),
    };

    match result {
        Ok(Some(new_state)) => {
            *mode = new_state;
            unsafe { out_bus.write(&sm2m::Output::ok()) };
        }
        Ok(None) => unsafe { out_bus.write(&sm2m::Output::ok()) },
        Err(err) => {
            error_led.set_low();
            unsafe { out_bus.write(&sm2m::Output::error()) };
            *mode = Mode::Error(err);
        }
    }

    cx.local.trigger.clear_interrupt_pending_bit();
}

fn mode_error<W: sm2m::Write>(err: &AppError, out_bus: &mut W) -> Result<Option<Mode>, AppError> {
    unsafe { out_bus.write(&sm2m::Output::data(err.opcode())) }
    Ok(Some(Mode::Ready))
}

pub fn cmd_unhandled() -> Result<Option<Mode>, AppError> {
    Ok(Some(Mode::Error(AppError::UnhandledCommand)))
}

pub fn open_sdmmc<'a>(
    card: &'a mut sdmmc::Card,
) -> Result<(sdmmc::card::Controller<'a>, embedded_sdmmc::Volume), AppError> {
    let block_spi = card.acquire()?;
    let time_source = sdmmc::StaticTimeSource::default();
    let mut controller = embedded_sdmmc::Controller::new(block_spi, time_source);
    let volume = controller.get_volume(embedded_sdmmc::VolumeIdx(0))?;
    Ok((controller, volume))
}
